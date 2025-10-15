using System;
using System.IO;
using System.Runtime.InteropServices;

namespace PF.Bindings
{
    public static class PfBindings
    {
        static PfBindings()
        {
            // For .NET Standard 2.0, we rely on the runtime to find the native library
            // in the appropriate platform-specific subdirectory
        }

        #if NET5_0_OR_GREATER
        [DllImport("pf_bindings_csharp", CallingConvention = CallingConvention.Cdecl)]
#elif NETSTANDARD2_0
        [DllImport("libpf_bindings_csharp", CallingConvention = CallingConvention.Cdecl)]
#else
        [DllImport("pf_bindings_csharp", CallingConvention = CallingConvention.Cdecl)]
#endif
        private static extern void pf_verify_bet(string receipt_json, string transcript_json, out IntPtr error);

#if NET5_0_OR_GREATER
        [DllImport("pf_bindings_csharp", CallingConvention = CallingConvention.Cdecl)]
#elif NETSTANDARD2_0
        [DllImport("libpf_bindings_csharp", CallingConvention = CallingConvention.Cdecl)]
#else
        [DllImport("pf_bindings_csharp", CallingConvention = CallingConvention.Cdecl)]
#endif
        private static extern void pf_register_gdp_package(byte[] bytes, IntPtr length, out IntPtr error);

        public static void VerifyBet(string receiptJson, string transcriptJson)
        {
            if (receiptJson == null) throw new ArgumentNullException(nameof(receiptJson));
            if (transcriptJson == null) throw new ArgumentNullException(nameof(transcriptJson));

            pf_verify_bet(receiptJson, transcriptJson, out IntPtr errorPtr);
            
            if (errorPtr != IntPtr.Zero)
            {
                var errorMsg = Marshal.PtrToStringAnsi(errorPtr);
                throw new Exception($"Verification failed: {errorMsg}");
            }
        }

        public static void RegisterGdpPackage(byte[] bytes)
        {
            if (bytes == null) throw new ArgumentNullException(nameof(bytes));

            pf_register_gdp_package(bytes, (IntPtr)bytes.Length, out IntPtr errorPtr);
            
            if (errorPtr != IntPtr.Zero)
            {
                var errorMsg = Marshal.PtrToStringAnsi(errorPtr);
                throw new Exception($"Registration failed: {errorMsg}");
            }
        }
    }
}
