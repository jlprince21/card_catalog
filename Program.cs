using System;
using System.IO;

using MySql.Data;
using MySql.Data.MySqlClient;

namespace file_scanner
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");

            DirectoryInfo di = new DirectoryInfo(@"/home/jlprince21/Desktop/stuff/coding/file_scanner");

            FileInfo[] fileList = di.GetFiles("*", SearchOption.AllDirectories);

            foreach (FileInfo file in fileList)
            {
                // Console.WriteLine(file.Name);
            }

            WriteToDB();

            Console.ReadLine();
        }

        static void WriteToDB()
        {
            string connStr = "server=localhost;user=root;database=metaverse;port=3306;password=XXXXXXXXXXXXXXXX;SslMode=none";
            MySqlConnection conn = new MySqlConnection(connStr);
            try
            {
                Console.WriteLine("Connecting to MySQL...");
                conn.Open();

                // string sql = "SELECT Name, HeadOfState FROM Country WHERE Continent='Oceania'";
                string sql = "SELECT * from Listings";
                MySqlCommand cmd = new MySqlCommand(sql, conn);
                MySqlDataReader rdr = cmd.ExecuteReader();

                while (rdr.Read())
                {
                    Console.WriteLine(rdr[0] + " -- " + rdr[1]);
                }
                rdr.Close();
            }
            catch (Exception ex)
            {
                Console.WriteLine(ex.ToString());
            }

            conn.Close();
            Console.WriteLine("Done.");
        }
    }
}
