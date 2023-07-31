namespace ConsoleApp1
{
    public class Solution
    {
        public int[] PlusOne(int[] digits)
        {
            int[] digitsClone = new int[digits.Length];
            Array.Copy(digits, 0, digitsClone, 0, digits.Length);

            for (int i = digitsClone.Length - 1; i >= 0; i--)
            {
                if (digitsClone[i] != 9)
                {
                    digitsClone[i]++;
                    break;
                }
                else
                {
                    digitsClone[i] = 0;
                }
            }

            if (digitsClone[0] == 0)
            {
                int[] temp = new int[digitsClone.Length + 1];
                temp[0] = 1;
                return temp;
            }

            return digitsClone;
        }

        public static int Main()
        {
            Console.WriteLine("Hello World");
            Solution solution = new();
            int[] array = { 1, 2, 3 };
            int[] answer = solution.PlusOne(array);

            foreach (var item in answer)
            {
                Console.WriteLine(item.ToString());
            }

            return 0;
        }
    }
}