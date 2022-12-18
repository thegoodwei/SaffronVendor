pragma solidity ^0.7.0;

import "https://github.com/OpenZeppelin/openzeppelin-solidity/contracts/token/ERC20/SafeERC20.sol";
import "https://github.com/OpenZeppelin/openzeppelin-solidity/contracts/math/SafeMath.sol";

contract SaffronVendor {
  using SafeMath for uint256;
  using SafeERC20 for IERC20;

  string public name = "Saffron Vendor";
  string public symbol = "$affron";
  uint8 public decimals = 18;
  uint256 public totalSupply = 10000;
  IERC20 public affron;
  uint256 public paymentReceived = 0;

  constructor() public {
    affron = new IERC20(this);
    affron.mint(totalSupply);
  }

  event Bought(uint256 countBuys);
  event Redeem(uint256 redeemSaffron, uint256 paymentAmount);
//buyAffron function receives payment in ETH and sends back 1 $affron token to the caller. The function also increments the paymentReceived variable and emits a Bought event with the current value of paymentReceived.

  function buyAffron() public payable {
    require(msg.value > 0, "Payment must be greater than zero");
    require(affron.transfer(msg.sender, 1), "Transfer failed");
    paymentReceived = paymentReceived.add(msg.value);
    emit Bought(paymentReceived);
  }
//redeemAffron function allows the contract owner to redeem $affron tokens and burn them. The function first checks that the contract has at least 1 $affron token to redeem, then transfers 1 $affron token to the caller and burns it. The function also emits a Redeem event with the current balance of $affron tokens on the contract and the total paymentReceived value.

  function redeemAffron() public {
    require(affron.balanceOf(address(this)) > 0, "There are no Saffron tokens to redeem");
    require(affron.transfer(msg.sender, 1), "Redeem failed");
    affron.burn(1);
    emit Redeem(affron.balanceOf(address(this)), paymentReceived);
    paymentReceived = 0;
  }
}
