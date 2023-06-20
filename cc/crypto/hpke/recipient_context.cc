/*
 * Copyright 2023 The Project Oak Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "cc/crypto/hpke/recipient_context.h"

namespace oak::crypto {

absl::StatusOr<KeyPair> KeyPair::Generate() {
  // TODO(#4026): Generate a key pair using BoringSSL.
  std::string private_key = "";
  std::string public_key = "";
  return KeyPair{private_key, public_key};
}

}  // namespace oak::crypto