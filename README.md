# Bonding Curve Program

This program implements a bonding curve logic for calculating token prices based on the available supply and demand. It is written in Anchor for Solana smart contract development.

## Formula

The bonding curve uses the following formula to determine the price:

### **Price Calculation Formula**
For each token amount `amount` purchased:
- Remaining supply: `remaining_supply = initial_supply - amount`
- Supply ratio: `supply_ratio = remaining_supply / initial_supply`
- Price: `price = base_price * e^(k * (1 - supply_ratio))`

Where:
- `k` is a constant that defines the steepness of the curve.
- `e` is Euler's number (approximately 2.718).

### **Cost Calculation**
The program calculates the cost for purchasing tokens in small steps to ensure accurate pricing:
1. Divide the `amount` into small `steps`.
2. For each step, calculate the price based on the supply at that step.
3. Sum up the costs for all steps.

### **Key Functions**
1. `calculate_price_original`:
   - Computes the price based on the remaining supply, using the bonding curve formula.

2. `calculate_price_fixed`:
   - Divides the purchase into steps, calculates the price for each step, and sums the total cost.

## How It Works
- The program ensures dynamic pricing based on token availability.
- It is designed to be efficient, breaking down large purchases into smaller increments for precision.

This logic can be used in DeFi applications where token price needs to adjust dynamically based on the supply.
