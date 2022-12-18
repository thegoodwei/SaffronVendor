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

  function buyAffron() public payable {
    require(msg.value > 0, "Payment must be greater than zero");
    require(affron.transfer(msg.sender, 1), "Transfer failed");
    paymentReceived = paymentReceived.add(msg.value);
    emit Bought(paymentReceived);
  }

  function redeemAffron() public {
    require(affron.balanceOf(address(this)) > 0, "There are no Saffron tokens to redeem");
    require(affron.transfer(msg.sender, 1), "Redeem failed");
    affron.burn(1);
    emit Redeem(affron.balanceOf(address(this)), paymentReceived);
    paymentReceived = 0;
  }
}
