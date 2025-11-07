#include "webview/webview.h"
#include <windows.h>
#include <iostream>
#include <thread>
#include <string>

static WNDPROC g_orig_wndproc = nullptr;
static bool g_is_fullscreen = false;
static DWORD g_saved_style = 0;
static DWORD g_saved_exstyle = 0;
static RECT g_saved_rect = {0};

void toggle_fullscreen(HWND hwnd) {
    if (!IsWindow(hwnd)) return;

    if (!g_is_fullscreen) {
        // salvar estado atual
        g_saved_style = GetWindowLong(hwnd, GWL_STYLE);
        g_saved_exstyle = GetWindowLong(hwnd, GWL_EXSTYLE);
        GetWindowRect(hwnd, &g_saved_rect);

        // remover decoração e colocar no topo
        DWORD newStyle = g_saved_style & ~(WS_OVERLAPPEDWINDOW);
        SetWindowLong(hwnd, GWL_STYLE, newStyle);
        SetWindowLong(hwnd, GWL_EXSTYLE, g_saved_exstyle & ~(WS_EX_DLGMODALFRAME | WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE | WS_EX_STATICEDGE));

        // posiciona fullscreen (cobre monitor primário onde a janela está)
        HMONITOR hmon = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        MONITORINFO mi = { sizeof(mi) };
        if (GetMonitorInfo(hmon, &mi)) {
            SetWindowPos(hwnd, HWND_TOP,
                         mi.rcMonitor.left, mi.rcMonitor.top,
                         mi.rcMonitor.right - mi.rcMonitor.left,
                         mi.rcMonitor.bottom - mi.rcMonitor.top,
                         SWP_FRAMECHANGED | SWP_SHOWWINDOW);
        } else {
            SetWindowPos(hwnd, HWND_TOP, 0, 0, GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN),
                         SWP_FRAMECHANGED | SWP_SHOWWINDOW);
        }

        g_is_fullscreen = true;
    } else {
        // restaurar
        SetWindowLong(hwnd, GWL_STYLE, g_saved_style);
        SetWindowLong(hwnd, GWL_EXSTYLE, g_saved_exstyle);

        SetWindowPos(hwnd, HWND_NOTOPMOST,
                     g_saved_rect.left, g_saved_rect.top,
                     g_saved_rect.right - g_saved_rect.left,
                     g_saved_rect.bottom - g_saved_rect.top,
                     SWP_FRAMECHANGED | SWP_SHOWWINDOW);

        g_is_fullscreen = false;
    }
}

LRESULT CALLBACK hook_wndproc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
    if (msg == WM_KEYDOWN) {
        if (wp == VK_F11) {
            toggle_fullscreen(hwnd);
            return 0; // consumiu
        }
        // opcional: Esc sai do fullscreen
        if (wp == VK_ESCAPE && g_is_fullscreen) {
            toggle_fullscreen(hwnd);
            return 0;
        }
    }

    // Caso você queira capturar ALT+ENTER ou outras combinações:
    if (msg == WM_SYSKEYDOWN) {
        if (wp == VK_RETURN && (GetKeyState(VK_MENU) & 0x8000)) { // Alt+Enter
            toggle_fullscreen(hwnd);
            return 0;
        }
    }

    return CallWindowProc(g_orig_wndproc, hwnd, msg, wp, lp);
}

extern "C" __declspec(dllexport) void run_webview()
{
    webview::webview webView(false, nullptr);

    HWND hwnd = (HWND)webView.window().value();

    // Substitui WndProc (subclass)
    g_orig_wndproc = (WNDPROC)SetWindowLongPtr(hwnd, GWLP_WNDPROC, (LONG_PTR)hook_wndproc);
    if (!g_orig_wndproc) {
        std::cerr << "Aviso: SetWindowLongPtr retornou NULL (pode ser erro ou valor zero).\n";
    }

    webView.set_title("Subway Surfers");
    webView.set_size(1213, 720, WEBVIEW_HINT_NONE);
    webView.navigate("http://localhost:6967");
    webView.run();

    if (g_orig_wndproc) {
        SetWindowLongPtr(hwnd, GWLP_WNDPROC, (LONG_PTR)g_orig_wndproc);
    }
}