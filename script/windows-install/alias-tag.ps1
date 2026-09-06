# Inspect the OS reparse tag without following a user-created symbolic link.
param([string]$Path)
if (-not ('CodeTether.InstallAlias' -as [type])) {
    Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
using Microsoft.Win32.SafeHandles;
namespace CodeTether {
    public static class InstallAlias {
        [DllImport("kernel32.dll", CharSet=CharSet.Unicode, SetLastError=true)]
        static extern SafeFileHandle CreateFile(string name, uint access, uint share,
            IntPtr security, uint creation, uint flags, IntPtr template);
        [DllImport("kernel32.dll", SetLastError=true)]
        static extern bool GetFileInformationByHandleEx(SafeFileHandle file,
            int infoClass, out TagInfo info, uint size);
        [StructLayout(LayoutKind.Sequential)]
        struct TagInfo { public uint Attributes; public uint Tag; }
        public static bool IsAppExecLink(string path) {
            using (var file = CreateFile(path, 0, 7, IntPtr.Zero, 3, 0x02200000, IntPtr.Zero)) {
                TagInfo info;
                if (file.IsInvalid || !GetFileInformationByHandleEx(file, 9, out info, 8))
                    throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error());
                return info.Tag == 0x8000001b;
            }
        }
    }
}
'@
}
if (-not [CodeTether.InstallAlias]::IsAppExecLink($Path)) { throw 'PACKAGE_ALIAS_NOT_APPEXECLINK' }
