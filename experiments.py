from math import sqrt
import numpy as np

profit_history = np.array([1.98, -4.31, 32.58])
cum_profit_history = np.array([1.98, -2.33, 30.25])
net_profit_history = np.array([
    0,
    0,
    0,
    1.98,
    1.98,
    1.98,
    1.98,
    1.98,
    1.98,
    1.98,
    -2.33,
    -2.33,
    -2.33,
    -2.33,
    -2.33,
    -2.33,
    -2.33,
    -2.33,
    -2.33,
    30.25
])
open_profit_history = np.array([
    0,
    -25.11,
    2.15,
    0,
    0,
    0,
    0,
    5.26,
    4.83,
    -4.8,
    0,
    0,
    0,
    -0.72,
    -4.5,
    4.7,
    29.01,
    32.94,
    32.75,
    0
])
equity_history = np.array([
    1000000,
    999974.89,
    1000002.15,
    1000001.98,
    1000001.98,
    1000001.98,
    1000001.98,
    1000007.24,
    1000006.81,
    999997.18,
    999997.67,
    999997.67,
    999997.67,
    999996.95,
    999993.17,
    1000002.37,
    1000026.68,
    1000030.61,
    1000030.42,
    1000030.25
])

first_trade_entry_date = "2015-11-11"
first_trade_close_date = "2015-11-13"

last_trade_entry_date = "2015-11-23"
last_trade_close_date = "2015-11-29"


def compute_sharpe(mr: float, sd: float, rfr: float):
    return (mr - rfr) / sd


# MR = SHARPE * SD + RFR
def compute_mr_from_sharpe(sharpe_ratio: float, sd: float, rfr: float):
    return sharpe_ratio * sd + rfr


# SD = (MR - RFR) / SHARPE
def compute_sd_from_sharpe(sharpe_ratio: float, mr: float, rfr: float):
    return (mr - rfr) / sharpe_ratio


# MR = SORTINO * DD + RFR
def compute_mr_from_sortino(sortino_ratio: float, dd: float, rfr: float):
    return sortino_ratio * dd + rfr


# DD = (MR - RFR) / SORTINO
def compute_dd_from_sortino(sortino_ratio: float, mr: float, rfr: float):
    return (mr - rfr) / sortino_ratio


def compute_returns(returns: np.ndarray) -> np.ndarray:
    returns_shifted = np.roll(returns, 1)
    _returns = (returns / returns_shifted) - 1
    _returns = _returns[1:]
    return _returns
    return np.cumprod(1 + returns) - 1


def compute_sortino(mr: float, rfr: float, dd: float):
    return (mr - rfr) / dd


# MR = SHARPE * SD + RFR
# MR = SORTINO * DD + RFR
# SHARPE * SD = SORTINO * DD

# SHARPE        DD
# ------   =    ---
# SORTINO       SD
#
#
# 0.216 / 1.61 = DD / SD(0%)
#

# 0.216 * S = 1.61 * D
# -7.206 * S + 2 = -0.991 * D + 2
# -14.627 * S + 4 = -0.998 * D + 4
# -18.337 * S + 5 = -0.999 * D + 5


# Equity: 1000, 1001.98, 997.67, 1030.25
_equity_returns = np.array([0.00198, -0.0043, 0.032656])
#
# 0.216 * SD = 0.208 * SD + 2
#
# 0.216 / 0.208 = 2 / ( 0.208 * SD )
# 0.216 / 0.208 = 2 / 0.208 * SD
# 1.0384615 = 9.61538 / SD
# SD = 9.259255

# RFR = 0.0
target_sharpe_ratio_rfr_0 = 0.216  # (MR - RFR) / SD
target_sortino_ratio_rfr_0 = 1.618  # (MR - RFR) / DD
# RFR = 2 (2%)
target_sharpe_ratio_rfr_2 = 0.208
target_sortino_ratio_rfr_2 = 1.541


# print(np.var(_equity_returns))

# 1.828

days = 18
mr = np.sum(_equity_returns, axis=0) / days

var = np.var(_equity_returns, axis=0)
sd = np.std(_equity_returns, axis=0)

# 1000, 995.69, 1032.58
# -0.00431

# sqrt((-0.00431 - 0.0)^2 / 2)
# sqrt(-0.02431^2 / 2)
#
#
#
#
#
#
#

# print(compute_sortino(mr, sd, 0.0))
#

# RFR = 4 (4%)
# target_sharpe_ratio_rfr_4 = -14.627
# target_sortino_ratio_rfr_4 = -0.998
# # RFR = 5 (5%)
# target_sharpe_ratio_rfr_5 = -18.337
# target_sortino_ratio_rfr_5 = -0.999

# profit_returns = profit_history
# profit_returns = compute_returns(cum_profit_history)
# profit_returns_mean = np.mean(profit_returns)
# profit_returns_std = np.std(profit_returns)


# print(compute_sharpe(profit_returns_mean, profit_returns_std, 0.0))
# print(compute_sharpe(profit_returns_mean, profit_returns_std, 0.02))

# print(compute_sortino(12, 0.0, 15))

# sqrt(sum(min(0, Xi - T))^2/N), where Xi - ith return, N - total number of returns, T - target return.
#


def dd(returns: np.ndarray, rfr: float):
    n = len(returns)
    t = rfr
    return np.sqrt(np.sum(np.power(np.minimum(returns - t, 0), 2)) / n)


def mm(returns: np.ndarray, days: int):
    sum = np.sum(returns)
    return np.mean(returns)
    return sum / days


# sortino: 0.391, rfr = 0%
# Equity: 100, 85.06, 132.58
# Returns: -0.1494, 0.558664
_returns = np.array([-4.43, 10.05])
_returns = np.array([-14.94, 38.3])

_target = 0.00
_dd = dd(_returns, _target)
_mm = mm(_returns, 29 - 14)


def compute_return(current: float, prev: float):
    return (current / prev) - 1.0


first_equity = 100
second_equity = 100

_daily_profit = np.array([
    0,
    compute_return(first_equity + -3.98, first_equity),
    compute_return(first_equity + -17.7, first_equity),
    compute_return(first_equity + -6.15, first_equity),
    compute_return(first_equity + -0.9, first_equity),
    compute_return(first_equity + -1.33, first_equity),
    compute_return(first_equity + -10.96, first_equity),
    compute_return(first_equity + -14.94, first_equity),
    0,
    0,
    compute_return(second_equity + -0.72, second_equity),
    compute_return(second_equity + -4.5, second_equity),
    compute_return(second_equity + 4.7, second_equity),
    compute_return(second_equity + 29.01, second_equity),
    compute_return(second_equity + 32.94, second_equity),
    compute_return(second_equity + 32.75, second_equity),
    compute_return(second_equity + 0, second_equity),
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
])
_mm = np.mean(_daily_profit)
print(_daily_profit)
print(_mm)

_sd = np.std(_returns)

# sqrt(sum(min(0, Xi - T))^2/N)
#

print(compute_sortino(_mm, _target, _sd))

# # sortino: 0.351, rfr = 50%
# # Equity: 100, 85.06, 132.58
# # Returns: -0.1494, 0.558664
# print(dd(_returns, 0.5))

# Sharpe RFR 0% = 0.144
# Sharpe RFR 60% = 0.127

# sharpe = (MR - RFR) / SD


# var long_entries = array.from(1515, 1524)
# var long_exits = array.from(1522, 1530)
# 0.144 = (MR - 0) / SD
# 0.141 = (MR - 10) / SD
# 0.138 = (MR - 20) / SD
# 0.135 = (MR - 30) / SD
# 0.133 = (MR - 40) / SD
# 0.130 = (MR - 50) / SD
# 0.127 = (MR - 60) / SD
# 0.122 = (MR - 80) / SD
# 0.117 = (MR - 100) / SD
# MR = (533;535
# 53.24 * 10 = 532.4
# Sum returns: 23.36
# Mean Returns: 11.68
# Days: 29-14 = 15

# var long_entries = array.from(1512, 1518, 1524)
# var long_exits = array.from(1514, 1521, 1530)
# MR = 597.5
# Sum Returns: 31.11
# Mean Returns: 10.37

597 = 31.11 / x

x = MEAN / MA

#
