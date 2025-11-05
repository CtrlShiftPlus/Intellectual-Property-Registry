# Intellectual Property Registry Smart Contract

## Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Future Scope](#future-scope)

## Project Title:
**Intellectual Property Registry**

## Project Description:
This smart contract implements a simple registry for managing intellectual property records, where users can create, retrieve, update, and delete records related to intellectual property (e.g., patents, trademarks, copyrights). It allows users to store a title, description, and a unique ID for each intellectual property record, ensuring that each entry is identifiable and securely stored on the blockchain.

## Project Vision:
The **Intellectual Property Registry** aims to provide a decentralized, transparent, and tamper-proof way of storing and managing intellectual property records. By using blockchain technology, the project ensures the security, authenticity, and immutability of these records, making it easier for creators, developers, and businesses to prove ownership of intellectual property assets.

The vision for this project is to create a trusted and easily accessible platform for users to register and verify intellectual property ownership. This solution could be particularly useful in industries such as entertainment, technology, and research, where intellectual property rights are crucial for protecting ideas and inventions.

## Key Features:
1. **Create IP Record**: Users can create a new intellectual property record with a title and description.
2. **Get IP Record**: Users can retrieve an existing intellectual property record by its unique ID.
3. **Update IP Record**: Users can update the description of an existing intellectual property record.
4. **Delete IP Record**: Admin can delete an existing intellectual property record by its unique ID.
5. **Immutability**: Once a record is created, it cannot be altered except through an update function, ensuring the integrity of the data.

## Future Scope:
- **Ownership Verification**: Integrate ownership verification mechanisms, allowing users to prove that they own the IP registered in the system.
- **Multi-user Access**: Allow for multiple parties (e.g., joint inventors or creators) to manage a single intellectual property record.
- **Smart Contract Extensions**: Add more functionality, such as a transfer mechanism or dispute resolution feature for IP owners.
- **IP Monetization**: Integrate with platforms that allow users to monetize their IPs by licensing or selling their intellectual property.
- **Integration with Legal Systems**: Work towards integrating the registry with legal systems for more formal recognition of blockchain-based intellectual property records.

---

### How to Interact with the Contract:
To interact with the contract, use the following methods:
1. **create_ip_record(title: String, description: String)** - Creates a new IP record.
2. **get_ip_record(unique_id: u64)** - Retrieves an IP record by its unique ID.
3. **update_ip_record(unique_id: u64, new_description: String)** - Updates the description of an IP record.
4. **delete_ip_record(unique_id: u64)** - Deletes an IP record by its unique ID.

You can use the Soroban SDK's built-in methods to deploy and interact with this contract using the terminal or through a dApp.

---

### License:
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


# Contract Details
Contract ID:
CC5XFFA2O4Z3CHQ75INWKGLKKBGGLQ4A42BK5FJXN3M7H7UOMI5T5A6J

<img width="1909" height="912" alt="image" src="https://github.com/user-attachments/assets/57c4580c-735b-47f5-8418-87b5d590d554" />

