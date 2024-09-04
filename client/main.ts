
  import {
    Connection,
    Keypair,
    PublicKey,
    TransactionMessage,
    VersionedTransaction,
    SystemProgram,
    TransactionInstruction,
    LAMPORTS_PER_SOL,
    Transaction,
    sendAndConfirmTransaction,
  
  } from "@solana/web3.js";
  import {deserialize, deserializeUnchecked, serialize } from "borsh";
  import { UserAccount, UserAccountShema, Book, BookShema, Config, ConfigShema,BookCount, BookCountShema, UserCount, UserCountShema, BookNumber, BookNumberShema} from "./models";
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");

  const privatekey = [209,202,75,77,51,59,102,81,8,45,50,58,209,54,134,238,29,107,221,66,98,156,30,20,186,236,255,189,136,8,36,169,49,191,167,29,47,172,73,19,16,188,51,135,9,154,137,226,181,182,26,127,251,38,99,119,117,149,77,134,182,216,216,215]
  const payer = Keypair.fromSecretKey(Uint8Array.from(privatekey));
 
  const program_id =  new PublicKey("FGJrp5TSKHBmVupJFHHyW2bPr6rcE9Y25oz65gX5jZsq");
  const user = new PublicKey("5ccyMorkWF46spggcrmocX7e7moM9cYCuAAELf2G3qjs");
  const book = new PublicKey("2hzmLcMw6b3LphJJxW1dKrLGnsRUVPMktusDF4hTreQr");
  const user_count = new PublicKey("3z529VSwDGfx3PkzQnH5G8wHN4myfcuo8GA1xDyafAja");
  const book_count = new PublicKey("BF7AwTBZAYi7R93gFMrsRiAnd8xssNvxKoh92HyUHiTH");
  const config = new PublicKey("Ffg81T9my3WcbnMG7sFrhajsUpYFyYcNE7oj6bU7Aqza");

  const user_count_acc = async () => {
    const user_count = new UserCount();
    user_count.user_counter = BigInt(0);

    const encoded = serialize(UserCountShema,user_count);
    const concat = Uint8Array.of(0, ...encoded);

    const counterPDA = PublicKey.findProgramAddressSync([Buffer.from("c")],program_id)

    const instruction = new TransactionInstruction({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        {pubkey: counterPDA[0], isSigner: false, isWritable: true}, // counterPDA[0]-> publickey counterPDA[1] -> bump dondurur
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
      ],
      data: Buffer.from(concat),
      programId: program_id
    })

    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    
    const tx = new VersionedTransaction(message);
     tx.sign([payer]);
  
    connection.sendTransaction(tx);
    console.log("User Counter => " + counterPDA[0].toString())
    // console.log("User Bump => " + counterPDA[1].toString())
  }

  const create_user_acc = async () => {
    const user = new UserAccount();

    const borrowDate = BigInt(Math.floor(Date.now() / 1000));
    const returnDate = borrowDate + BigInt(10 * 24 * 60 * 60);

    user.user_no = BigInt(1);
    user.user_address =  new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
      16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);
    user.book_id = 0;
    user.borrow_date = borrowDate;
    user.return_date = returnDate; 
     
    const encoded = serialize(UserAccountShema,user);
    const concat = Uint8Array.of(1, ...encoded);

    const user_count_read = await connection.getAccountInfo(user_count); // user count'u okuduk
    const user_count_deserizalize = deserialize(UserCountShema, UserCount, user_count_read!.data);

    const convert_type: bigint =  BigInt (user_count_deserizalize.user_counter ); 
    const add = BigInt(1) + convert_type;

    const userPDA = PublicKey.findProgramAddressSync([Buffer.from("user"), Buffer.from(add.toString() )], program_id)

    const instruction = new TransactionInstruction({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        {pubkey: user_count, isSigner: false, isWritable: true},
        {pubkey: config, isSigner: false, isWritable: false},
        {pubkey: userPDA[0], isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},

      ],
      data: Buffer.from(concat),
      programId: program_id
    })

    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    
    const tx = new VersionedTransaction(message);
     tx.sign([payer]);
  
    connection.sendTransaction(tx);
    console.log("New users account => " + userPDA[0])
    

  }

  const read_user = async () => {

  const user_information = await connection.getAccountInfo(user)
  const user_inf_data = deserialize(UserAccountShema, UserAccount, user_information?.data!);

  console.log(user_inf_data.user_no.toString())
  console.log(user_inf_data.user_address)
  console.log(user_inf_data.book_id.toString())
  console.log(user_inf_data.borrow_date.toString())
  console.log(user_inf_data.return_date.toString())

}

  const book_count_acc = async () => {
    const book_count = new BookCount();
    book_count.book_counter= 0;

    const encoded = serialize(BookCountShema,book_count);
    const concat = Uint8Array.of(2, ...encoded);

    const book_counter_PDA = PublicKey.findProgramAddressSync([Buffer.from("bookcounter")], program_id);

    const instruction = new TransactionInstruction({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        {pubkey: book_counter_PDA[0], isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
      ],
      data: Buffer.from(concat),
      programId: program_id
    })

    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    
    const tx = new VersionedTransaction(message);
     tx.sign([payer]);
  
    connection.sendTransaction(tx);
    console.log("Book Counter => " + book_counter_PDA[0].toString())
    console.log("Book count => " + book_count.book_counter)

    const deserialize_book_count_account = deserialize(BookCountShema, BookCount, Buffer.from(encoded)); 
  }
  
  const create_book_acc = async () => {
    const book = new Book();
    book.book_id = 0;
    book.in_circulation = 0;
    book.total_Number_Of_Books = 10;

  const encoded = serialize(BookShema, book);
  const concat = Uint8Array.of(3, ...encoded);

  const book_count_read = await connection.getAccountInfo(book_count);
  const book_count_deserizalize = deserialize(BookCountShema, BookCount, book_count_read!.data);

  const convert_type: bigint =  BigInt (book_count_deserizalize.book_counter ); 
  const add = BigInt(1) + convert_type;

  const bookPDA = PublicKey.findProgramAddressSync([Buffer.from("Createbook"), Buffer.from(add.toString())], program_id);
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: payer.publicKey, isSigner: true, isWritable: true},
      {pubkey: config, isSigner: false, isWritable: false},
      {pubkey: book_count, isSigner: false, isWritable: true},
      {pubkey: bookPDA[0], isSigner: false, isWritable: true},
      {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ],
    data: Buffer.from(concat),
    programId: program_id
  })

  const message = new TransactionMessage({
    instructions: [instruction],
    payerKey: payer.publicKey,
    recentBlockhash: (await connection.getLatestBlockhash()).blockhash
  }).compileToV0Message();

  
  const tx = new VersionedTransaction(message);
   tx.sign([payer]);

  connection.sendTransaction(tx);

  console.log(book_count_deserizalize.book_counter)
  console.log(add.toString())

  console.log("Book account => " + bookPDA[0].toString())

   }

   const read_book = async () => {
    
    const book_information = await connection.getAccountInfo(book)
    const book_inf_data = deserializeUnchecked(BookShema, Book, book_information?.data!);
  
    console.log("book type => " + book_inf_data.book_id.toString())
    console.log("total number of books => " + BigInt(book_inf_data.total_Number_Of_Books).toString());
    console.log("number of books in circulation => " + BigInt(book_inf_data.in_circulation).toString());
  
  
  }
  
   const delete_book_acc = async () => {
    const book = new Book();
    book.book_id = 0;
    book.in_circulation = 0;
    book.total_Number_Of_Books = 10;

  const encoded = serialize(BookShema, book);
  const concat = Uint8Array.of(4, ...encoded);

  const book_count_read = await connection.getAccountInfo(book_count);
  const book_count_deserizalize = deserialize(BookCountShema, BookCount, book_count_read!.data);

  const convert_type: bigint =  BigInt (book_count_deserizalize.book_counter ); 
  const add = BigInt(1) + convert_type;

  const deletebookPDA = PublicKey.findProgramAddressSync([Buffer.from("bookDelete"), Buffer.from(add.toString())], program_id);
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: payer.publicKey, isSigner: true, isWritable: true},
      {pubkey: config, isSigner: false, isWritable: false},
      {pubkey: book_count, isSigner: false, isWritable: true},
      {pubkey: deletebookPDA[0], isSigner: false, isWritable: true},
      {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ],
    data: Buffer.from(concat),
    programId: program_id
  })

  const message = new TransactionMessage({
    instructions: [instruction],
    payerKey: payer.publicKey,
    recentBlockhash: (await connection.getLatestBlockhash()).blockhash
  }).compileToV0Message();

  
  const tx = new VersionedTransaction(message);
   tx.sign([payer]);

  connection.sendTransaction(tx);
  console.log("Book account => " + deletebookPDA[0].toString())
   }

   const borrow_book = async() => {
    
   const instruction = new TransactionInstruction({ 
    keys: [
      {pubkey: payer.publicKey, isSigner: true, isWritable: true},
      {pubkey: user, isSigner: false, isWritable: true }, 
      {pubkey: book ,isSigner: false, isWritable: true}, 
    ],

    data: Buffer.from([5]),
    programId: program_id
  })
  
  const message = new TransactionMessage({
    instructions: [instruction],
    payerKey: payer.publicKey,
    recentBlockhash: (await connection.getLatestBlockhash()).blockhash
  }).compileToV0Message();

  
  const tx = new VersionedTransaction(message);
   tx.sign([payer]);
 
   connection.sendTransaction(tx);
   console.log("Barrow the book");

   }

   const return_book = async() => {
   
   const instruction = new TransactionInstruction({ 
    keys: [
      {pubkey: payer.publicKey, isSigner: true, isWritable: true},
      {pubkey: user, isSigner: false, isWritable: true }, 
      {pubkey: book ,isSigner: false, isWritable: true}, 
      {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
    ],

    data: Buffer.from([6]),
    programId: program_id
  })
  

  const message = new TransactionMessage({
    instructions: [instruction],
    payerKey: payer.publicKey,
    recentBlockhash: (await connection.getLatestBlockhash()).blockhash
  }).compileToV0Message();

  
  const tx = new VersionedTransaction(message);
   tx.sign([payer]);
 
   connection.sendTransaction(tx);
   console.log("Return the book");

  }

  const add_book_acc = async(data:bigint) => {

    const book_num = new BookNumber()
    book_num.booknumber = data;

    const encoded = serialize(BookNumberShema, book_num);
    const concat = Uint8Array.of(7, ...encoded);

    const instruction = new TransactionInstruction({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        {pubkey: book, isSigner: false, isWritable: true},
        {pubkey: config, isSigner: false, isWritable: false},
      ],
      data:Buffer.from(concat),
      programId:program_id
    })
    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    
    const tx = new VersionedTransaction(message);
     tx.sign([payer]);
  
    connection.sendTransaction(tx);
    console.log("New book added");
  
  }
  
  const remove_book_acc = async(data:bigint) => {

    const book_number = new BookNumber()
    book_number.booknumber = data;

    const encoded = serialize(BookNumberShema, book_number);
    const concat = Uint8Array.of(8, ...encoded);

    const instruction = new TransactionInstruction({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        {pubkey: book, isSigner: false, isWritable: true},
        {pubkey: config, isSigner: false, isWritable: false},
      ],
      data:Buffer.from(concat),
      programId:program_id
    })
    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    
    const tx = new VersionedTransaction(message);

    tx.sign([payer]);
  
    connection.sendTransaction(tx);
    console.log("Remove book");
  
  }

  const createConfigAccount = async () => {

    const configPDA = PublicKey.findProgramAddressSync([Buffer.from("config")], program_id);
    const instruction = new TransactionInstruction({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        {pubkey: configPDA[0], isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
      ],
      data: Buffer.from([9]),
      programId: program_id
    })
  
    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    
    const tx = new VersionedTransaction(message);
     tx.sign([payer]);
  
    connection.sendTransaction(tx);
    console.log(configPDA[0].toString())
  }


  // user_count_acc()
  // create_user_acc()
  // read_user()
  // book_count_acc()
  // create_book_acc()
  // read_book()
  // delete_book_acc()
  // borrow_book()
  // return_book()
  // add_book_acc(BigInt(3))
  // remove_book_acc(BigInt(1))
  // createConfigAccount()

