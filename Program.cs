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

            DirectoryInfo di = new DirectoryInfo(@"/mnt/unraid");

            FileInfo[] fileList = di.GetFiles("*", SearchOption.AllDirectories);

            Console.WriteLine("Files gathered, commencing DB updates...");

            foreach (FileInfo file in fileList)
            {
                // Console.WriteLine(file.Name);
                WriteToDB($@"INSERT INTO `Listings`(`FileName`, `FilePath`, `Checksum`, `FileSize`, `ChecksumDate`) VALUES ('{EscapeString(file.Name)}', '{EscapeString(file.Directory.ToString())}', 'test', {file.Length}, null)");
            }

            Console.WriteLine("DB updates completed.");

            // WriteToDB();

            Console.ReadLine();
        }

        static string EscapeString(string toEscape)
        {
            if (toEscape.Contains("'")) {
                string escaped = toEscape.Replace("'", "''");
                return escaped;
            } else {
                return toEscape;
            }
        }

        static void WriteToDB(string sql)
        {
            string connStr = "server=localhost;user=root;database=metaverse;port=3306;password=XXXXXXXXXXXXXXXX;SslMode=none";
            MySqlConnection conn = new MySqlConnection(connStr);
            try
            {
                // Console.WriteLine("Connecting to MySQL...");
                conn.Open();

                // string sql = "SELECT Name, HeadOfState FROM Country WHERE Continent='Oceania'";
                // string sql = "SELECT * from Listings";
                MySqlCommand cmd = new MySqlCommand(sql, conn);
                MySqlDataReader rdr = cmd.ExecuteReader();

                // while (rdr.Read())
                // {
                //     Console.WriteLine(rdr[0] + " -- " + rdr[1]);
                // }
                rdr.Close();
            }
            catch (Exception ex)
            {
                Console.WriteLine("Failure: " + sql);
                Console.WriteLine(ex.ToString());
            }

            conn.Close();
            // Console.WriteLine("Done.");
        }
    }
}
