CREATE TABLE `Listings` (
 `guid` int(10) unsigned NOT NULL AUTO_INCREMENT,
 `file_name` mediumtext NOT NULL,
 `file_path` mediumtext NOT NULL,
 `checksum` mediumtext,
 `file_size` bigint(20) unsigned NOT NULL,
 `checksum_date` datetime DEFAULT NULL,
 UNIQUE KEY `GUIDPrimary` (`guid`)
) ENGINE=InnoDB AUTO_INCREMENT=132375 DEFAULT CHARSET=utf8mb4