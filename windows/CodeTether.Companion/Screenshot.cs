using System.Drawing.Drawing2D;
using System.Drawing.Imaging;
using System.Security.Cryptography;

namespace CodeTether.Companion;

/// <summary>Captures only the locally selected monitor; pixels never touch disk.</summary>
internal static class Screenshot
{
    public static FrameBody Capture(Rectangle bounds, string trigger, string? requestId)
    {
        if (bounds.Width <= 0 || bounds.Height <= 0 || (long)bounds.Width * bounds.Height > 34000000)
            throw new InvalidOperationException("Unsupported monitor size.");
        using Bitmap source = new(bounds.Width, bounds.Height, PixelFormat.Format24bppRgb);
        using (Graphics desktop = Graphics.FromImage(source))
            desktop.CopyFromScreen(bounds.Location, Point.Empty, bounds.Size, CopyPixelOperation.SourceCopy);
        double scale = Math.Min(1, 1600d / Math.Max(bounds.Width, bounds.Height));
        using Bitmap resized = new(Math.Max(1, (int)(bounds.Width * scale)), Math.Max(1, (int)(bounds.Height * scale)));
        using (Graphics graphics = Graphics.FromImage(resized))
        {
            graphics.InterpolationMode = InterpolationMode.HighQualityBicubic;
            graphics.DrawImage(source, new Rectangle(Point.Empty, resized.Size));
        }
        ImageCodecInfo jpeg = ImageCodecInfo.GetImageEncoders().Single(codec => codec.FormatID == ImageFormat.Jpeg.Guid);
        foreach (long quality in new long[] { 75, 55, 35, 20 })
        {
            using MemoryStream bytes = new();
            using EncoderParameters options = new(1);
            options.Param[0] = new EncoderParameter(System.Drawing.Imaging.Encoder.Quality, quality);
            resized.Save(bytes, jpeg, options);
            if (bytes.Length > 524288) continue;
            string image = Convert.ToBase64String(bytes.GetBuffer(), 0, (int)bytes.Length);
            CryptographicOperations.ZeroMemory(bytes.GetBuffer());
            using (Graphics clear = Graphics.FromImage(source)) clear.Clear(Color.Black);
            using (Graphics clear = Graphics.FromImage(resized)) clear.Clear(Color.Black);
            return new FrameBody(image, DateTimeOffset.UtcNow.ToString("O"), trigger, requestId);
        }
        throw new InvalidOperationException("Screenshot exceeds the upload limit.");
    }
}