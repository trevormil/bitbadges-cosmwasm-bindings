use cosmwasm_std::QuerierWrapper;
use crate::query::BitBadgesQuery;
// msg::{BadgeCollection, Balance}};

pub struct BitBadgesQuerier<'a> {
    querier: &'a QuerierWrapper<'a, BitBadgesQuery>,
}

impl<'a> BitBadgesQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, BitBadgesQuery>) -> Self {
        BitBadgesQuerier { querier }
    }

    // pub fn query_get_collection<T: Into<String>>(&self, id: u64) -> StdResult<BadgeCollection> {
    //     let request = BitBadgesQuery::QueryCollection { 
    //         id
    //     }
    //     .into();

    //     self.querier.query(&request)
    // }

    // pub fn query_address_by_id<T: Into<String>>(&self, id: u64) -> StdResult<String> {
    //     let request = BitBadgesQuery::QueryAddressById { 
    //         id
    //     }
    //     .into();

    //     self.querier.query(&request)
    // }

    // pub fn query_balance<T: Into<String>>(&self, badge_id: u64, address: String) -> StdResult<Balance> {
    //     let request = BitBadgesQuery::QueryBalance { 
    //         badge_id,
    //         address
    //     }
    //     .into();

    //     self.querier.query(&request)
    // }
}
