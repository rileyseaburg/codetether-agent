using System.ComponentModel;
using System.Runtime.InteropServices;

namespace CodeTether.Companion;

/// <summary>Recognizes clicks only. No keystrokes, click history, or coordinates are transmitted.</summary>
internal sealed partial class MouseHook : IDisposable
{
    private readonly HookCallback callback;
    private readonly Action<string> trigger;
    private IntPtr handle;
    private uint lastTime;
    private Point lastPoint;
    private bool hasLast;
    public MouseHook(Action<string> trigger) { this.trigger = trigger; callback = Handle; }
    public void Start()
    {
        if (handle != IntPtr.Zero) return;
        handle = SetWindowsHookEx(14, callback, IntPtr.Zero, 0);
        if (handle == IntPtr.Zero) throw new Win32Exception(Marshal.GetLastWin32Error());
    }
    private IntPtr Handle(int code, IntPtr message, IntPtr data)
    {
        if (code >= 0)
        {
            MouseData click = Marshal.PtrToStructure<MouseData>(data);
            if ((click.Flags & 1) == 0 && message.ToInt64() == 0x0205) trigger("right_click");
            if ((click.Flags & 1) == 0 && message.ToInt64() == 0x0201)
            {
                Size distance = SystemInformation.DoubleClickSize;
                bool twice = hasLast && unchecked(click.Time - lastTime) <= GetDoubleClickTime() &&
                    Math.Abs((long)click.X - lastPoint.X) <= distance.Width / 2 &&
                    Math.Abs((long)click.Y - lastPoint.Y) <= distance.Height / 2;
                hasLast = !twice; lastTime = click.Time; lastPoint = new Point(click.X, click.Y);
                if (twice) trigger("double_click");
            }
        }
        return CallNextHookEx(handle, code, message, data);
    }
    public void Dispose()
    {
        if (handle != IntPtr.Zero) UnhookWindowsHookEx(handle);
        handle = IntPtr.Zero; hasLast = false; lastPoint = Point.Empty; lastTime = 0;
    }
}