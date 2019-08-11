select
    listing.id as listing_id,
    listing.checksum,
    listing.file_name,
    listing.file_path,
    listing.file_size,
    listing_tag.id as listing_tag_id,
    tag.id as tag_id,
    tag.tag
from
    listing
    inner join
        listing_tag
        on (listing.id = listing_tag.listing_id)
    inner join
        tag
        on (listing_tag.tag_id = tag.id);