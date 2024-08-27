/*****************************************************************************
 *   Ledger App Ironfish Rust.
 *   (c) 2023 Ledger SAS.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *****************************************************************************/

use alloc::vec;
use alloc::vec::Vec;
use ironfish_frost::participant::Secret as ironfishSecret;
use crate::AppSW;
use ledger_device_sdk::ecc::{Secret, bip32_derive, CurvesId, ChainCode};
use ledger_device_sdk::io::Comm;

pub fn handler_dkg_get_identity(comm: &mut Comm) -> Result<(), AppSW> {
    let path_0: Vec<u32> = vec![(0x80000000 | 0x2c), (0x80000000 | 0x53a), (0x80000000 | 0x0), (0x80000000 | 0x0),
            (0x80000000 | 0x00)];
    let path_1: Vec<u32> = vec![(0x80000000 | 0x2c), (0x80000000 | 0x53a), (0x80000000 | 0x0), (0x80000000 | 0x0),
                               (0x80000000 | 0x01)];

    let mut secret_key_0 = Secret::<64>::new();
    let mut secret_key_1 = Secret::<64>::new();
    let mut cc: ChainCode = Default::default();

    // Ignoring 'Result' here because known to be valid
    let _ = bip32_derive(
        CurvesId::Ed25519,
        &path_0,
        secret_key_0.as_mut(),
        Some(cc.value.as_mut()),
    );
    let _ = bip32_derive(
        CurvesId::Ed25519,
        &path_1,
        secret_key_1.as_mut(),
        Some(cc.value.as_mut()),
    );

    let secret = ironfishSecret::from_secret_keys(
        secret_key_0.as_ref()[0..32].try_into().unwrap(),
        secret_key_1.as_ref()[0..32].try_into().unwrap()
    );

    let identity = secret.to_identity();
    comm.append(identity.serialize().as_ref());

    Ok(())
}
