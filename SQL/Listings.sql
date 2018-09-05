CREATE TABLE `Listings` (
 `GUID` int(10) unsigned NOT NULL AUTO_INCREMENT,
 `FileName` mediumtext NOT NULL,
 `FilePath` mediumtext NOT NULL,
 `Checksum` mediumtext,
 `FileSize` bigint(20) unsigned NOT NULL,
 `ChecksumDate` datetime DEFAULT NULL,
 UNIQUE KEY `GUIDPrimary` (`GUID`)
) ENGINE=InnoDB AUTO_INCREMENT=88568 DEFAULT CHARSET=utf8mb4