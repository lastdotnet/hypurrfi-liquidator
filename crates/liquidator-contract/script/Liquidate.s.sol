pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console.sol";

import {Liquidator} from "src/Liquidator.sol";

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
    RunLocals memory locals; 

    // Start broadcasting transactions
    vm.startBroadcast(vm.envUint("PRIVATE_KEY"));
    
    // Get the collateral token and debt token addresses
    locals.collateralToken = vm.envAddress("COLLATERAL");
    locals.debtToken = vm.envAddress("DEBT");
    
    // Read liquidation parameters from .env.liquidation file
    locals.userToLiquidate = vm.envAddress("BORROWER");
    locals.debtToCover = vm.envUint("DEBT_TO_COVER");

    locals.liquidator = new Liquidator();

    // approve the pool
    locals.liquidator.approvePool(locals.debtToken);
    
    // Call liquidation function - adjust parameters based on your protocol's implementation
    (locals.finalToken, locals.collateralToReceive) = locals.liquidator.liquidate(
        locals.collateralToken,
        locals.debtToken,
        locals.userToLiquidate,
        locals.debtToCover,
        vm.envBytes("SWAP_PATH"),
        vm.envString("LIQ_PATH")
    );

    console.log('swap path: ');
    console.logBytes(vm.envBytes("SWAP_PATH"));

    console.log('collateralToReceive: ');
    console.logInt(locals.collateralToReceive);

    console.log('finalToken: ');
    console.logAddress(locals.finalToken);

    vm.stopBroadcast();
  }
}