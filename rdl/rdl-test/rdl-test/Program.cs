using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace rdl_test
{
    class Program
    {
        [DllImport("rdl.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern void process();

        static void Main(string[] args)
        {
            Stopwatch watch = new Stopwatch();
            watch.Start();



            //Parallel.For(0, 4, i =>
            //{
            //var x = 0;
            //for (int j = 0; j < 500000000; j++)
            //{
            //    x += 1;
            //}
            //    Console.WriteLine("线程:{0} 完成计数", Thread.CurrentThread.ManagedThreadId);
            //});

            process();//调用Rust里面的程序process进行计算

            watch.Stop();
            Console.WriteLine("耗时:{0}秒", watch.Elapsed.TotalSeconds);
            Console.Read();
        }
    }
}
