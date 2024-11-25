/*
* Metered Subscription is the intended usage with monthly subscription being the main alternative in the form of. But to make it easier for regular users to use the service it also offers basic and premium plans.
*/
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, strum::Display, strum::EnumString, Serialize, Deserialize,
)]
pub enum PaymentPlan {
    Free,
    //MonthlyBasic,
    //MonthlyPremium,
    MeteredSubscription,
    MonthlySubscription,
    OneTime,
    Canceled, // When using any subscription type and the user want's to cancel it. An update account with payment plan as canceled is requested.
}

/*
* https://stripe.com/en-se/guides/payment-methods-guide
*/
#[derive(
    Debug, Clone, Eq, PartialEq, strum::Display, strum::EnumString, Serialize, Deserialize,
)]
pub enum PaymentMethod {
    Card,
    Wallet,
    BankDebit,
    //Crypto, // Support later, maybe?
}

/*
https://stripe.com/docs/products-prices/pricing-models#volume-tiers
User can only have one active subscription at a time.
either metered or subscription.
Both can not be active at the same time.
The user is able to do one time payments as well whenever.


The payment plans available will be

Pricing
- Metered. (Pay per usage)
- Subscription. (Cycle payment) 
- One Time Payment. (Single payment)

When ever a user uses a subscription or onetime-payment, then user balance is used.
When a user runs out of balance, they can no longer use services that cost.

metered subscription provide unlimited usage. But

*/
#[derive(
    Debug, Clone, Eq, PartialEq, strum::Display, strum::EnumString, Serialize, Deserialize,
)]
pub enum PaymentModel {
    Metered,
    Subscription,
    OneTime,
}