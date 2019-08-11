SELECT * FROM listing
WHERE
    checksum IN (
        SELECT
            checksum
        FROM (
            SELECT
                checksum,
                ROW_NUMBER()
                OVER (PARTITION BY
                        checksum
                    ORDER BY
                        id ASC) AS Row
                FROM
                    listing) dups
            WHERE
                dups.Row > 1)