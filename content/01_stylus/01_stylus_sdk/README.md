# Stylus SDK

Stylus SDK는 Rust와 다른 WASM 호환 언어를 사용하여 스마트 계약을 개발할 수 있도록 설계된 강력한 도구 키트입니다. Ethereum 기반 개발을 위한 Rust 라이브러리인 Alloy 위에 구축된 Stylus SDK는 개발자들이 Arbitrum에서 고성능 탈중앙화 애플리케이션(dApp)을 손쉽게 개발할 수 있도록 돕습니다. 이 SDK는 기존 Rust 라이브러리와 원활하게 통합되며 효율적인 계약 개발, 배포 및 상호작용을 가능하게 하는 기능을 제공합니다.

Solidity 생태계에서 온 개발자들에게 익숙한 구문을 사용할 수 있는 기능은 큰 장점입니다. 이를 통해 Rust를 빠르게 이해하고 작업할 수 있어, 완전히 새로운 언어를 배우는 데 드는 시간을 줄일 수 있습니다. 또한 Stylus SDK는 Solidity와 유사한 프로그래밍 개념을 공유하므로, 개발자들이 Rust와 Solidity에 대한 지식을 활용하여 Stylus SDK를 다룰 수 있습니다.

## Solidity 스마트 계약

Solidity를 알고 있다면, 이 계약은 주소를 통해 소유권을 관리하는 기본적인 계약입니다. 여기에는 두 개의 주요 함수가 포함됩니다: 현재 소유자의 주소를 가져오는 함수와 새로운 주소로 소유자를 업데이트하는 함수입니다. 계약은 공용 상태 변수로 소유자의 주소를 저장하여 시스템의 다른 부분이 소유권을 수정하거나 상호작용할 수 있도록 합니다.

```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

contract Ownable {
    address public owner;

    function owner() public pure returns(address) {
        return owner;
    }

    function setOwner(address _owner) public {
        owner = _owner;
    }
}
```

## Stylus Rust SDK 스마트 계약

Stylus SDK의 **Ownable** 스마트 계약은 Solidity 버전과 매우 유사하게 소유권을 주소를 통해 관리하는 핵심 로직을 유지합니다. 이 Stylus 계약에서 `owner`는 소유자의 주소를 블록체인에 지속적으로 저장하는 특수 타입인 `StorageAddress`를 사용하여 저장됩니다. `owner()` 함수는 Solidity 계약의 getter 함수와 유사하게 현재 소유자의 주소를 반환하고, `set_owner()` 함수는 Solidity의 `setOwner()` 함수와 비슷하게 소유자의 주소를 업데이트할 수 있습니다.

```rust
use stylus_sdk::alloy_primitives::{Address};
use stylus_sdk::prelude::*;
use stylus_sdk::storage::{StorageAddress6};

#[storage]
#[entrypoint]
pub struct Ownable {
    owner: StorageAddress,
}

#[external]
impl Ownable {
    pub fn owner(&self) -> Address {
        self.owner.get()
    }

    pub fn set_owner(&self, _owner: StorageAddress) {
        self.owner.set(_owner);
    }
}
```

주요 차이점은 Stylus가 Rust 구문과 `#[storage]` 속성을 사용하여 `owner`를 저장 변수로 지정하고, `#[entrypoint]`는 계약의 진입점을 지정하는 방식에 있습니다. 이러한 차이점에도 불구하고, 소유권 관리 로직의 핵심은 거의 동일하게 유지되며, Stylus가 개발자들에게 Rust로 스마트 계약을 작성하면서 Solidity의 익숙한 패턴을 그대로 유지할 수 있게 하여, Solidity 개발자들이 Rust 기반 스마트 계약 개발로 쉽게 전환할 수 있도록 합니다.