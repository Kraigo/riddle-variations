using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace Variants
{

    public class Variants {
        public double[] Arr;
        public int Basis;
        public double VariantsSize;
        public int VariansCount;
        public Func<double, double[]> Convert;

        public Variants(double[] Arr, int Basis)
        {
            this.Arr = Arr;
            this.Basis = Basis;
            VariantsSize = Math.Pow((double)Arr.Length, (double)Basis);
            VariansCount = 0;
            Convert = Converter(10, Arr.Length);
        }

        public IEnumerator GetEnumerator()
        {
            for (VariansCount = 0; VariansCount < VariantsSize; VariansCount++)
            {
                yield return Convert(VariansCount)
                    .ToList()
                    .Select(k => (double)Arr.GetValue((int)k))
                    .ToArray();
            }
        }

        public Func<double, double[]> Converter(int from, int to)
        {
            return (double num) =>
            {
                var cur = num;
                var result = new double[Basis];
                var count = Basis - 1;
                while (cur != 0)
                {
                    var res = cur % to;
                    cur = Math.Floor(cur / to);
                    result[count--] = res;
                }
                return result;
            };
        }
    }

    class RiddleVariations
    {
        private double[] source;
        private double TARGET;
        private int AFTER_POINT;
        public RiddleVariations(double[] source, double target, int afterPoint)
        {
            this.source = source;
            TARGET = target;
            AFTER_POINT = afterPoint;
        }

        public double[][] Calc()
        {
            var sourceModified = new List<double>();
            var variants = new List<double[]>();

            for (var i = 0; i < source.Length; i++)
            {
                for (var n = 0; n < source.Length; n++)
                {

                    var twoDigit = String.Format("{0}{1}", source[i], source[n]);
                    double d;
                    if (double.TryParse(twoDigit, out d))
                    {
                        sourceModified.Add(d);
                    }
                }
                for (var p = 1; p <= AFTER_POINT; p++)
                {
                    var pointVariants = new Variants(source, p);
                    foreach (double[] v in pointVariants)
                    {
                        var withPoint = String.Format("{0}.{1}", source[i], String.Join(String.Empty, v));
                        double d;
                        if (double.TryParse(withPoint, out d))
                        {
                            sourceModified.Add(d);
                        }
                    }
                }
            }
            source = sourceModified.Concat(source)
                .ToList()
                .Where(item => item < TARGET)
                .Distinct()
                .ToArray();

            var sourceVariants = new Variants(source, 3);

            Console.WriteLine("Variants items " + source.Length);
            Console.WriteLine("Variants count " + sourceVariants.VariantsSize);

            foreach (double[] variant in sourceVariants)
            {
                var sum = variant.Sum();
                if (sum == TARGET)
                {
                    variants.Add(variant);
                }
            }

            return variants
                .Select(r =>
                {
                    Array.Sort(r);
                    return String.Join(',', r);
                })
                .Distinct()
                .Select(r => r.Split(',')
                    .ToList()
                    .Select(i => double.Parse(i))
                    .ToArray())
                .ToArray();
        }
    }
    
    class Program
    {
        static void Main(string[] args)
        {

            var source = new double[] { 1, 3, 5, 7, 9, 11, 13, 15 };
            var target = 30;
            var afterPoint = 1;
            var riddleVariations = new RiddleVariations(source, target, afterPoint);
            var result = riddleVariations.Calc();

            Console.WriteLine($"Result {result.Count()}");

            //foreach (var r in result)
            //{
            //    Console.WriteLine($"{String.Join(" + ", r)} = {TARGET}");
            //}
            Console.ReadLine();
        }
    }
}
