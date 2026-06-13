# ChainSubscription Hub

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)

**ChainSubscription Hub** is a decentralized smart contract platform designed to manage subscription plans and user subscriptions with automated renewals. Built using Soroban on the Stellar blockchain, it provides transparent management, enforcing subscription rules, renewal cycles, and pausing or canceling subscriptions securely and trustlessly.

---

## 🎯 Project Vision

The vision of ChainSubscription Hub is to offer subscription-based businesses a decentralized, secure, and automated way to handle user subscriptions and renewals without relying on centralized intermediaries. This guarantees transparent access management and payment enforcement, ultimately increasing user trust and business reliability.

## 🔗 Contract Details

* **Platform:** Stellar (Soroban Smart Contracts)
* **Contract ID:** `CBRIWWK6PV7NW5UAEMHSTLECFDDD7XVZWYEZHV3MCSOOPXDBCZO5BA5I`

## ✨ Key Features

* **Plan Management:** Admins can easily create and manage subscription plans, specifying duration and pricing.
* **User Subscriptions:** Users can subscribe to available plans with flexible options to enable auto-renewal.
* **Automated Renewal:** Subscriptions automatically renew based on the plan's duration and payment status (payment logic handled externally).
* **Subscription Cancellation:** Users have the autonomy to cancel subscriptions, disabling immediate or future renewals.
* **Immutable Records:** All plans and subscription states are permanently recorded on-chain, ensuring complete auditability.
* **Access Control:** Strictly enforces admin-restricted plan management and user-restricted subscription control.
* **Transparent Status:** Subscription statuses and histories are publicly accessible for maximum transparency.

## 🛠 Technology Stack

* **Rust & Soroban SDK:** For secure, highly optimized smart contract development.
* **Stellar Blockchain:** For decentralized, lightning-fast, and immutable state management.
* **Cryptography:** Utilizing cryptographic signing and timestamping for secure subscription enforcement.

## 📖 Usage Instructions

1. **Set Admin:** Deploy the contract and assign an admin address for subscription management.
2. **Create Plans:** The assigned Admin adds subscription plans by defining the name, price, and duration.
3. **Subscribe:** Users interact with the contract to subscribe to plans and can optionally enable the auto-renew feature.
4. **Auto Renew:** The auto-renewal process can be triggered either by an external scheduler or via direct user action.
5. **Cancel:** Users can easily cancel their active subscriptions whenever desired.
6. **Query:** Anyone can query the smart contract to read subscription details and verify transparency.

## 🚀 Future Scope

* **Payment Integration:** Integrate Soroban native tokens or off-chain payment oracles to strictly enforce subscription payments.
* **Multi-tier Subscriptions:** Support for multi-level access or bundled subscription plans.
* **Trial Periods:** Introduce free trial subscriptions and support for discount promo codes.
* **User Dashboards:** Build intuitive web interfaces for both users and admins to seamlessly manage subscriptions.
* **Notification System:** Implement alerts for upcoming renewals, successful payments, cancellations, or payment failures.
* **Cross-Platform Sync:** Synchronize subscription statuses across various decentralized services and DApps.
* **Compliance Tools:** Automate tax calculation and regulatory compliance reporting.

## 🤝 Contribution

Community contributions are highly welcomed from blockchain developers and subscription platform experts! 

To contribute:
1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.

## 📄 License

This project is licensed under the **MIT License**.