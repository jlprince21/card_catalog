select
    listings.id as listing_id,
    listings.checksum,
    listings.file_name,
    listings.file_path,
    listings.file_size,
    listing_tags.id as listing_tags_id,
    tags.id as tags_id,
    tags.tag
from
    listings
    inner join
        listing_tags
        on (listings.id = listing_tags.listing_id)
    inner join
        tags
        on (listing_tags.tag_id = tags.id);
--	where listings.id = '145ef410-81f5-4c41-948c-246951561f5b'

