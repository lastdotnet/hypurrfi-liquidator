pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console.sol";

import {Liquidator} from "src/Liquidator.sol";
import {IERC20} from "src/interfaces/IERC20.sol";

contract Default is Script {
  using stdJson for string;

  struct RunLocals {
    address collateralToken;
    address debtToken;
    address userToLiquidate;
    uint256 debtToCover;
    bytes swapPath;
    string liqPath;
    Liquidator liquidator;
    int256 collateralToReceive;
    address finalToken;
  }

  function run() external {    
    // Start broadcasting transactions
    vm.startBroadcast(vm.envUint("PRIVATE_KEY"));

    // check the user's balance
    uint256 userBalance = address(0x026dd7B8E6140c444278C219094004a3032AA6C1).balance;
    console.log("userBalance: ");
    console.log(userBalance);

    // check the user's balance of WHYPE
    uint256 whypeBalance = IERC20(0x5555555555555555555555555555555555555555).balanceOf(0x026dd7B8E6140c444278C219094004a3032AA6C1);
    console.log("whypeBalance: ");
    console.log(whypeBalance);

    vm.stopBroadcast();
  }
}