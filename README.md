# **Move** 🚀

Move is a budget planning application designed to empower users with the ability to track, analyze, and optimize their spending habits and financial goals.

## **Commands**

Move provides a variety of commands for managing the budget. Below are some key commands:

To use the application, first run the following command:

```
cargo build

cargo install --path . & cargo help
```

Once you run the above commands, you can then start working on this application by writing:

```
move --help
```

It will give you a list of all subcommands that you can execute like:

```
// Setup the total amount
move total-amount add amount --amount 500

// Add the category in the total amount list
move total-amount add category --category groceries --label "the list of groceries"

// Create a budget
move budget create --category groceries/utilities --amount 200

// View the budget info
move budget view --category groceries/utilities

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

We welcome contributions! If you have ideas for new features, find a bug, or want to improve documentation, feel free to open an issue or submit a pull request. Please follow our [Contribution Guidelines](contribution_guide.md) for a smooth collaboration.

## **License**

Move is licensed under the [Apache-2.0 License](LICENSE). Feel free to use, modify, and distribute the code as per the license terms.