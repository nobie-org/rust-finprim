#!/usr/bin/env python3
"""
Test Excel FV formula manually to understand the correct calculation.
"""

import math

def excel_fv(rate, nper, pmt, pv=0, type=0):
    """
    Excel FV function implementation
    
    Args:
        rate: Interest rate per period
        nper: Number of periods
        pmt: Payment per period
        pv: Present value (default 0)
        type: Payment timing (0=end of period, 1=beginning of period)
    
    Returns:
        Future value
    """
    if rate == 0:
        return pmt * nper + pv
    
    # Calculate (1 + rate)^nper
    compound_factor = (1 + rate) ** nper
    
    # Calculate annuity factor: [(1 + rate)^nper - 1] / rate
    annuity_factor = (compound_factor - 1) / rate
    
    # Future value of present value
    fv_of_pv = pv * compound_factor
    
    # Future value of annuity
    if type == 1:  # Beginning of period
        fv_of_annuity = pmt * annuity_factor * (1 + rate)
    else:  # End of period
        fv_of_annuity = pmt * annuity_factor
    
    return fv_of_annuity + fv_of_pv

# Test cases from Microsoft Excel documentation
test_cases = [
    {
        "name": "Microsoft Example 1: FV(0.06/12, 10, -200, -500, 1)",
        "rate": 0.06/12,
        "nper": 10,
        "pmt": -200,
        "pv": -500,
        "type": 1,
        "expected": 2581.40
    },
    {
        "name": "Microsoft Example 2: FV(0.12/12, 12, -1000)",
        "rate": 0.12/12,
        "nper": 12,
        "pmt": -1000,
        "pv": 0,
        "type": 0,
        "expected": 12682.50
    },
    {
        "name": "Simple test: FV(0.05, 10, -100)",
        "rate": 0.05,
        "nper": 10,
        "pmt": -100,
        "pv": 0,
        "type": 0,
        "expected": 1257.78925  # From rust_finprim original test
    }
]

print("Testing Excel FV formula implementation:")
print("=" * 60)

for case in test_cases:
    result = excel_fv(case["rate"], case["nper"], case["pmt"], case["pv"], case["type"])
    expected = case["expected"]
    diff = abs(result - expected)
    
    print(f"\n{case['name']}")
    print(f"Parameters: rate={case['rate']:.6f}, nper={case['nper']}, pmt={case['pmt']}, pv={case['pv']}, type={case['type']}")
    print(f"Calculated: {result:.2f}")
    print(f"Expected:   {expected:.2f}")
    print(f"Difference: {diff:.2f}")
    print(f"Match: {'✓' if diff < 1.0 else '✗'}")

print("\n" + "=" * 60)
print("Manual calculation for Microsoft Example 1:")
rate = 0.06/12
nper = 10
pmt = -200
pv = -500
type = 1

print(f"rate = {rate:.6f}")
print(f"nper = {nper}")
print(f"pmt = {pmt}")
print(f"pv = {pv}")
print(f"type = {type}")

compound_factor = (1 + rate) ** nper
annuity_factor = (compound_factor - 1) / rate
fv_of_pv = pv * compound_factor
fv_of_annuity = pmt * annuity_factor * (1 + rate)  # type=1
total_fv = fv_of_annuity + fv_of_pv

print(f"\nStep by step:")
print(f"compound_factor = (1 + {rate:.6f})^{nper} = {compound_factor:.6f}")
print(f"annuity_factor = ({compound_factor:.6f} - 1) / {rate:.6f} = {annuity_factor:.6f}")
print(f"fv_of_pv = {pv} * {compound_factor:.6f} = {fv_of_pv:.2f}")
print(f"fv_of_annuity = {pmt} * {annuity_factor:.6f} * (1 + {rate:.6f}) = {fv_of_annuity:.2f}")
print(f"total_fv = {fv_of_annuity:.2f} + {fv_of_pv:.2f} = {total_fv:.2f}")