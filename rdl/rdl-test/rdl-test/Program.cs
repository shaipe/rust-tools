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

    [StructLayout(LayoutKind.Sequential)]
    public struct SampleStruct
    {
        public Int16 field_one;
        public Int32 field_two;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct test
    {
        public byte isbool;
    }

    class StringArguments
    {
        [DllImport("rdl", EntryPoint = "how_many_characters")]
        public static extern uint HowManyCharacters(string s);

        static public void xain()
        {
            var count = StringArguments.HowManyCharacters("göes to élevên");
            Console.WriteLine(count);
        }
    }

    class Program
    {
        [DllImport("rdl.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern void process();

        [DllImport("rdl.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern string test(string text);

        [DllImport("rdl.dll")]
        private static extern Int32 add_numbers(Int32 number1, Int32 number2);

        //[DllImport("rdl.dll", CallingConvention = CallingConvention.Cdecl)]
        //public static extern bool verify_form_sign(string url_params, string secret);

        //[DllImport("rdl.dll", CallingConvention = CallingConvention.Cdecl)]
        //public static extern string md5(string text);
        [DllImport("rdl.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern string string_from_rust(string s);

        [DllImport("rdl.dll", CallingConvention = CallingConvention.Cdecl)]
        static extern void printc(string str);


        [DllImport("rdl.dll")]
        private static extern SampleStruct get_simple_struct();

        [DllImport("rdl.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern Int32 count_substrings(string value, string substr);

        [DllImport("rdl.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern Boolean verify_test(string value, string substr);

        static void Main(string[] args)
        {
            Stopwatch watch = new Stopwatch();
            watch.Start();

            Console.WriteLine(count_substrings("banana", "na"));

            Console.WriteLine(verify_test("banana", "banana"));

            printc("testxxxx sdsd");

            var simple_struct = get_simple_struct();
            Console.WriteLine(simple_struct.field_one);
            Console.WriteLine(simple_struct.field_two);



            string xss = string_from_rust("this is a test");
            Console.WriteLine(xss);

            string ss = "appId=4a364f2d1f1fb842&fromFKFlag=1&FromFKId=18022&method=vast.order.retreat.count&proprietor=2&proprietorId=747&timestamp=1563524211&Token=4B38F1DF6FF1A96C2B30C18D4EB4B340B9D2DE9D34AB8BA38BD9D144739E76570D00955D2FFF001B4AEFB019A23CED33264AC72CB92114C67A25E2EF3A62B98E5109A77C22798FFA7F807032912BAAE8DD1CFCFA07C477796BA9DB949ACAF68DA080681DAAC3AC82C03EC78583B072A807A879DFB5B2A007BB8AFB49662487DAEDE44C34E684867FDB2B3561F23B57E28D94444D6B790E9045755D3C3F05DE017EC2436C993464B457A126E43B54D23450BFA1DCA0296E8CAC650E35A98E215AAA50B5514EDAF09BDAD6CDE650FB77FCEAA721667F7F267BA444900A9114B59595AC0C8ED6E4666995B79A0AD6B45DBEC559BCF9A58CB91D7FE5733C7D64038E1A532DC72A0A1EF19A230E14D846F5D5D2375D358EE66BEA&v=3.0&secret=0a2b7ae94a364f2d1f1fb8423e9efd00";

            //verify_form_sign(ss, "0a2b7ae94a364f2d1f1fb8423e9efd00");

            //md5("test");
            //Parallel.For(0, 4, i =>
            //{
            //var x = 0;
            //for (int j = 0; j < 500000000; j++)
            //{
            //    x += 1;
            //}
            //    Console.WriteLine("线程:{0} 完成计数", Thread.CurrentThread.ManagedThreadId);
            //});

            int x = add_numbers(100, 1000);
            Console.WriteLine("result {0}", x);

            //process();//调用Rust里面的程序process进行计算

            watch.Stop();
            Console.WriteLine("耗时:{0}秒", watch.Elapsed.TotalSeconds);
            Console.Read();
        }
    }
}
