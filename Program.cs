using System;
using System.IO;

using MySql.Data;
using MySql.Data.MySqlClient;

using System.Security.Cryptography;

namespace file_scanner
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Starting...");

            Console.ReadLine();
        }

        static void ScanFolder()
        {
            DirectoryInfo di = new DirectoryInfo(@"/mnt/unraid");

            FileInfo[] fileList = di.GetFiles("*", SearchOption.AllDirectories);

            Console.WriteLine("Files gathered, commencing DB updates...");

            foreach (FileInfo file in fileList)
            {
                WriteToDB($@"INSERT INTO `Listings`(`FileName`, `FilePath`, `Checksum`, `FileSize`, `ChecksumDate`) VALUES ('{EscapeString(file.Name)}', '{EscapeString(file.Directory.ToString())}', 'test', {file.Length}, null)");
            }

            Console.WriteLine("DB updates completed.");
        }
        static string HashFile(string path)
        {
            using (var md5 = MD5.Create())
            {
                using (var stream = File.OpenRead(path))
                {
                    var hash = md5.ComputeHash(stream);
                    return BitConverter.ToString(hash).Replace("-", "").ToLowerInvariant();
                }
            }
        }
        static string EscapeString(string toEscape)
        {
            if (toEscape.Contains("'"))
            {
                return toEscape.Replace("'", "''");
            }
            else
            {
                return toEscape;
            }
        }

        static void WriteToDB(string sql)
        {
            string connStr = "server=localhost;user=root;database=metaverse;port=3306;password=XXXXXXXXXXXXXXXX;SslMode=none";
            MySqlConnection conn = new MySqlConnection(connStr);
            try
            {
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
        }
    }
}
