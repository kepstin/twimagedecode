// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this
// file except in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use base64;
use chrono::{TimeZone, Utc};
use std::convert::TryInto;
use std::env;

const TWEPOCH: u64 = 1288834974657;

const WORKER_ID_BITS: u64 = 5;
const DATACENTER_ID_BITS: u64 = 5;
const SEQUENCE_BITS: u64 = 12;
const MAX_WORKER_ID: u64 = u64::max_value() ^ (u64::max_value() << WORKER_ID_BITS);
const MAX_DATACENTER_ID: u64 = u64::max_value() ^ (u64::max_value() << DATACENTER_ID_BITS);
const MAX_SEQUENCE_ID: u64 = u64::max_value() ^ (u64::max_value() << SEQUENCE_BITS);

const WORKER_ID_SHIFT: u64 = SEQUENCE_BITS;
const DATACENTER_ID_SHIFT: u64 = SEQUENCE_BITS + WORKER_ID_BITS;
const TIMESTAMP_LEFT_SHIFT: u64 = SEQUENCE_BITS + WORKER_ID_BITS + DATACENTER_ID_BITS;

fn main() {
    let mut arg = env::args().nth(1).expect("no twitter image id provided");
    if arg.len() != 15 {
        panic!("id is the wrong number of characters")
    }
    arg.push_str("A");
    let mut buf = [0; 12];

    base64::decode_config_slice(&arg, base64::URL_SAFE, &mut buf).expect("failed to decode base64");

    let snowflake = u64::from_be_bytes((&buf[0..8]).try_into().unwrap());

    let timestamp = Utc.timestamp_millis(
        ((snowflake >> TIMESTAMP_LEFT_SHIFT) + TWEPOCH)
            .try_into()
            .unwrap(),
    );
    let datacenter_id = (snowflake >> DATACENTER_ID_SHIFT) & MAX_DATACENTER_ID;
    let worker_id = (snowflake >> WORKER_ID_SHIFT) & MAX_WORKER_ID;
    let sequence = snowflake & MAX_SEQUENCE_ID;

    println!("snowflake: {:x}", snowflake);
    println!("timestamp: {}", timestamp);
    println!(
        "datacenter id: {}, worker id: {}, sequence_id: {}",
        datacenter_id, worker_id, sequence
    );
    println!("extra bytes: {:x?}", &buf[8..12]);
}
