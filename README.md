# **Move** 🚀

Move is a budget planning application that let users manage their budget by spending on different categories like groceries, utilities etc.

## **Operations**

It performs the following operations in pattern:

    ✅ First add the total amount
    ✅ Add the categories under the total amount
    ✅ Add budget by defining the categories
    ✅ Make the total amount status active
    ✅ Store the blockchain credentials
    ✅ The keys will be provided to use them later.
    ✅ Spend the money(in ethereum) on different categories.

You can also get the total amount, budget, and spending data in the CSV file for record saving.

## **Important crates**

Some of the crates that this project uses:

    ✅ Clap -- Command line parser
    ✅ Rusqlite -- Ergonomic wrapper for Sqlite
    ✅ Ethers -- An Ethereum library in Rust
    ✅ AES-GCM -- Encrypting and decrypting the data
    ✅ CSV -- CSV reader and writer crate
    ✅ Tokio -- Run the asynchronous functions
    ✅ Tabled -- Show the data in a nice table format

## **Commands**

Move provides a variety of commands for managing the budget. Below are some key commands:

To use the application, first run the following command:

```
✅ cargo build
✅ cargo install --path . & cargo help
```

Once you run the above commands, you can then start working on this application by writing:

```
move --help
```

It will give you a list of all subcommands that you can execute. Some of them are:

```
✅ Setup the total amount
move total-amount add amount --amount 500

✅ Add the category in the total amount list
move total-amount add category --category groceries --label "the list of groceries"

✅ Create a budget
move budget create --category groceries/utilities --amount 200

...
```

## **Cloning**

1. Fork the repository

2. Clone the repository:

```
git clone https://github.com/your-username/move.git
```

3. Navigate to the project directory:

```
cd move
```

## **Contributing**

We welcome contributions! If you have ideas for new features, find a bug, or want to improve documentation, feel free to open an issue or submit a pull request. Please follow our [Contribution Guidelines](CONTRIBUING.md) for a smooth collaboration.

## **License**

Move is licensed under the [Apache-2.0 License](LICENSE). Feel free to use, modify, and distribute the code as per the license terms.