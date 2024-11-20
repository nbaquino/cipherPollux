# Pollux Cipher

## Background
The Pollux Cipher is a cryptographic method that involves encoding messages in a unique way. It was developed as a variant of the Morse code where each symbol is replaced by a group of three digits. This cipher is particularly interesting because of its simplicity and its historical use in secure communications.

This project was developed as a machine problem for CMSC 191 - Computer Security and Intro to Cryptography.

## Approach

This Rust application implements text encoding and decoding using Morse code and the Pollux cipher. Here's a breakdown of the process:

1. **Morse Code Mapping**: The application first converts text into Morse code. Each character in the text is mapped to its corresponding Morse code sequence.

2. **Pollux Cipher Encoding**: The Morse code is then encoded using the Pollux cipher. In this cipher, each Morse code symbol (dot, dash, and space) is randomly mapped to a character from a predefined set. This mapping creates a numeric code that represents the original text in an encoded form.

3. **Decoding Process**: To decode the numeric code back to text, the application reverses the Pollux cipher mapping to retrieve the Morse code and then converts the Morse code back to the original text.

4. **Utility Functions**: The application includes functions to handle the conversion between text and Morse code, Morse code and numeric code, and vice versa. This modular approach ensures each step can be tested and maintained separately.

This approach allows for a secure method of text transmission, as the encoded message in numeric form does not directly reveal the original text or even the Morse code sequence unless the specific mappings used for encoding are known.

## Installation
To install the necessary components for this project, follow these steps for your operating system:

### Windows
1. Clone the repository:
   ```bash
   git clone https://github.com/nbaquino/pollux-cipher.git
   ```
2. Navigate to the project directory:
   ```bash
   cd pollux-cipher
   ```
3. Build the project:
   ```cmd
   cargo build
   ```

### Mac
1. Clone the repository:
   ```bash
   git clone https://github.com/nbaquino/pollux-cipher.git
   ```
2. Navigate to the project directory:
   ```bash
   cd pollux-cipher
   ```
3. Build the project:
   ```bash
   cargo build
   ```

### Linux
1. Clone the repository:
   ```bash
   git clone https://github.com/nbaquino/pollux-cipher.git
   ```
2. Navigate to the project directory:
   ```bash
   cd pollux-cipher
   ```
3. Build the project:
   ```bash
   cargo build
   ```

## Running
To run the project, execute the following command in the terminal:

```
cargo run
```
Make sure you are in the directory of the project file. 


## Contributing
We welcome contributions from the community. If you would like to contribute, please follow these steps:
1. Fork the repository.
2. Create a new branch for your feature (`git checkout -b feature/your_feature_name`).
3. Make your changes.
4. Submit a pull request.


For more information and updates, please check our [GitHub repository](https://github.com/nbaquino/pollux-cipher).
