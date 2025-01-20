Move is a budget planning application designed to empower users with the ability to track, analyze, and optimize their spending habits and financial goals.

With a user-friendly CLI. It manages the finances and achieve greater financial stability by leveraging different payment APIs for a comprehensive financial management solution.

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
// Initialize the application
move init -n username -g gmail-id -a app-password -o postgresql-host -p 5432 -u postgresql-user -w postgresql-password -d postgresql-dbname -s sslmode -k privatekey -e alchemy-url

// Create a budget
move budget create --category groceries/utilities --amount 300

// View the budget info
move budget view --category groceries/utilities

...
```

## **Cloning**

Clone the repository:

```
git clone https://github.com/ibilalkayy/move.git
```

Navigate to the project directory:

```
cd move
```

Create a `.env` file to put all your PostgresSQL credentials.

## **Contributing**

We welcome contributions! If you have ideas for new features, find a bug, or want to improve documentation, feel free to open an issue or submit a pull request. Please follow our Contribution Guidelines for a smooth collaboration.

## **License**

Move is licensed under the [Apache-2.0 License](LICENSE). Feel free to use, modify, and distribute the code as per the license terms.