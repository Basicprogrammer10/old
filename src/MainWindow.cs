using System;
using System.Drawing;
using System.Windows.Forms;

namespace Minecraft_ChunkControl
{
    public partial class MainWindow : Form
    {
        private int Mul = 18;
        private int Size = 15;
        private int OrgX = 100;
        private int OrgY = 100;

        private int x = 20;
        private int y = 20;

        private readonly Color BgColor = Color.FromArgb(46, 52, 64);
        private readonly Color FgColor = Color.FromArgb(191, 97, 106);

        public MainWindow()
        {
            InitializeComponent();
        }

        private void MainWindow_Paint(object sender, PaintEventArgs e)
        {
            //ClearColor(e);
            DrawArray();
        }

        private void DrawArray()
        {
            for (var I = 0; I < x; I++)
            {
                for (var J = 0; J < y; J++)
                {
                    DrawBox(I * Mul + OrgX - Left, J * Mul + OrgY - Top, Size, Size, FgColor);
                }
            }
        }

        private void ClearColor(PaintEventArgs e)
        {
            //e.Graphics.Clear(BgColor);
        }

        private void DrawBox(int x, int y, int w, int h, Color color)
        {
            var MyBrush = new SolidBrush(color);
            var FormGraphics = this.CreateGraphics();
            FormGraphics.FillRectangle(MyBrush, new Rectangle(x, y, w, h));
            MyBrush.Dispose();
            FormGraphics.Dispose();
        }

        private void MainWindow_MouseUp(object sender, MouseEventArgs e)
        {
            if (e.Button != MouseButtons.Right)
                return;
            timer1.Enabled = false;
            Cursor = Cursors.Arrow;
        }

        private void MainWindow_MouseDown(object sender, MouseEventArgs e)
        {
            if (e.Button != MouseButtons.Right)
                return;
            timer1.Enabled = true;
            Cursor = Cursors.SizeAll;
        }
        private void timer1_Tick(object sender, EventArgs e)
        {
            OrgX = Cursor.Position.X - x / 2 * Mul;
            OrgY = Cursor.Position.Y - y / 2 * Mul;
            Invalidate();
        }
    }
}