pragma solidity 0.4.24;

contract Test{

    uint256 public x;

    function set(uint256 _y) external{
        x = _y;
    }

    function get() external view returns(uint y){
        y=x;
    }

}