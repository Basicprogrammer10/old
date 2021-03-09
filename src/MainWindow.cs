using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Windows.Forms;

namespace Minecraft_ChunkControl
{
    public partial class MainWindow : Form
    {
        private int Mul = 27;
        private int orgX = 100;
        private int orgY = 100;

        private readonly Color BgColor = Color.FromArgb(46, 52, 64);
        private readonly Color FgColor = Color.FromArgb(191, 97, 106);

        public MainWindow()
        {
            InitializeComponent();
        }

        private void MainWindow_Paint(object sender, PaintEventArgs e)
        {
            ClearColor(e);
            drawArray();
        }

        private void drawArray()
        {
            for (int i = 0; i < 10; i++)
            {
                for (int j = 0; j < 10; j++)
                {
                    drawBox(i * Mul + orgX - Left, j * Mul + orgY - Top, 25, 25, FgColor);
                }
            }
        }

        private void ClearColor(PaintEventArgs e)
        {
            e.Graphics.Clear(BgColor);
        }

        private void drawBox(int x, int y, int w, int h, Color color)
        {
            System.Drawing.SolidBrush myBrush = new System.Drawing.SolidBrush(color);
            var formGraphics = this.CreateGraphics();
            formGraphics.FillRectangle(myBrush, new Rectangle(x, y, w, h));
            myBrush.Dispose();
            formGraphics.Dispose();
        }

        private void MainWindow_MouseUp(object sender, MouseEventArgs e)
        {
            timer1.Enabled = false;
        }

        private void MainWindow_MouseDown(object sender, MouseEventArgs e)
        {
            timer1.Enabled = true;
        }
        private void timer1_Tick(object sender, EventArgs e)
        {
            orgX = Cursor.Position.X;
            orgY = Cursor.Position.Y;
            Refresh();
        }
    }
}