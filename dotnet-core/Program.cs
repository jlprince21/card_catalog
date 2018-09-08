using System;
using System.IO;

using MySql.Data;
using MySql.Data.MySqlClient;

using System.Security.Cryptography;
using Microsoft.Extensions.Configuration;

namespace file_scanner
{
    class Program
    {
        // via https://docs.microsoft.com/en-gb/aspnet/core/fundamentals/configuration/index?view=aspnetcore-2.1&tabs=basicconfiguration
        public static IConfiguration Configuration { get; set; }
        public static string SQLConnectionString { get; set; }

        static void Main(string[] args)
        {
            Console.WriteLine("Starting...");

            var builder = new ConfigurationBuilder()
            .SetBasePath(Directory.GetCurrentDirectory())
            .AddJsonFile("settings.json");

            Configuration = builder.Build();

            SQLConnectionString = $@"server={Configuration["SQLServer"]};user={Configuration["SQLUser"]};database={Configuration["SQLDatabase"]};port={Configuration["SQLPort"]};password={Configuration["SQLPassword"]};SslMode=none";
            Console.WriteLine(SQLConnectionString);

            // TODO mathematically define data ranges to remove dependency on byte specifications
            // UpdateFileHashes(0, 1024);                   // 0 - 1kb
            // UpdateFileHashes(1024, 4096);                // 1kb - 4kb
            // UpdateFileHashes(4096, 16384);               // 4kb - 16kb
            // UpdateFileHashes(16384, 65536);              // 16kb - 64kb
            // UpdateFileHashes(65536, 262144);             // 64kb - 256kb
            // UpdateFileHashes(262144, 1048576);           // 256kb - 1mb
            // UpdateFileHashes(1048576, 4194304);          // 1mb - 4mb
            // UpdateFileHashes(4194304, 16777216);         // 4mb - 16mb
            // UpdateFileHashes(16777216, 67108864);        // 16mb - 64mb
            // UpdateFileHashes(67108864, 268435456)        // 64mb - 256mb
            // UpdateFileHashes(268435456, 1073741824);     // 256mb - 1gb
            UpdateFileHashes(1073741824, 4294967296);     // 1gb - 4gb

            Console.WriteLine("done");

            Console.ReadLine();
        }

        static void ScanFolder()
        {
            DirectoryInfo di = new DirectoryInfo($@"{Configuration["SQLServer"]}");

            FileInfo[] fileList = di.GetFiles("*", SearchOption.AllDirectories);

            Console.WriteLine("Files gathered, commencing DB updates...");

            foreach (FileInfo file in fileList)
            {
                WriteToDB($@"INSERT INTO `Listings`(`FileName`, `FilePath`, `Checksum`, `FileSize`, `ChecksumDate`) VALUES ('{EscapeString(file.Name)}', '{EscapeString(file.Directory.ToString())}', 'test', {file.Length}, null)");
            }

            Console.WriteLine("DB updates completed.");
        }

        static void UpdateFileHashes(ulong startSize, ulong endSize)
        {
            MySqlConnection conn = new MySqlConnection(SQLConnectionString);

            string sql = "";

            try
            {
                conn.Open();

                // string sql = "SELECT Name, HeadOfState FROM Country WHERE Continent='Oceania'";
                sql = $@"SELECT * FROM `Listings` WHERE `FileSize` >= {startSize} and `FileSize` <= {endSize} and ChecksumDate is NULL";
                MySqlCommand cmd = new MySqlCommand(sql, conn);
                MySqlDataReader rdr = cmd.ExecuteReader();

                while (rdr.Read())
                {
                    Console.WriteLine(rdr["GUID"] + " -- " + rdr[1]);
                    WriteToDB($@"UPDATE `Listings` set Checksum = '{HashFile(rdr["FilePath"] + "/" + rdr["FileName"])}', CheckSumDate = '2018-08-05 07:30:00' WHERE GUID = {rdr["GUID"]}");
                }
                rdr.Close();
            }
            catch (Exception ex)
            {
                Console.WriteLine("Failure: " + sql);
                Console.WriteLine(ex.ToString());
            }

            conn.Close();
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
            MySqlConnection conn = new MySqlConnection(SQLConnectionString);
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
