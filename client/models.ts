import { serialize, deserialize, Schema } from "borsh";

export class UserAccount { 
  user_no: bigint = BigInt(0);
  user_address: Uint8Array = new Uint8Array(32);
  book_id: number = 0;
  borrow_date: bigint = BigInt(0);
  return_date: bigint = BigInt(0);


  constructor(fields: { user_no: bigint; user_address:Uint8Array ; book_id:number; borrow_date:bigint; return_date:bigint;    } | undefined = undefined) {
    if (fields) {
      this.user_no = fields.user_no;
      this.user_address = fields.user_address;
      this.book_id = fields.book_id;
      this.borrow_date = fields.borrow_date;
      this.return_date = fields.return_date;

    }
  }
}

export const UserAccountShema = new Map([
  [UserAccount, {
    kind: "struct",
    fields: [
      ["user_no", "u64"],
      ["user_address", ["u8",32]],
      ["book_id", "u8"],
      ["borrow_date", "u64"],
      ["return_date", "u64"],
    ]
  }]
]);

export class UserCount  { 
  user_counter: bigint = BigInt(0);

  constructor(fields: { user_counter:bigint;} | undefined = undefined) {
    if (fields) {
      this.user_counter = fields.user_counter;
    }
  }
}

export const UserCountShema = new Map([
  [UserCount , {
    kind: "struct",
    fields: [
      ["user_counter", "u64"],
    ]
  }]
]);


export class BookNumber  { 
  booknumber: bigint = BigInt(0);

  constructor(fields: { booknumber:bigint;} | undefined = undefined) {
    if (fields) {
      this.booknumber = fields.booknumber;
    }
  }
}

export const BookNumberShema = new Map([
  [BookNumber , {
    kind: "struct",
    fields: [
      ["booknumber", "u64"],
    ]
  }]
]);


export class Book { 
    book_id: number = 0;
    total_Number_Of_Books: number = 0 ;
    in_circulation: number = 0 ;
  
    constructor(fields: { book_id: number; total_Number_Of_Books:number;  in_circulation:number  } | undefined = undefined) {
      if (fields) {
        this.book_id = fields.book_id;
        this.total_Number_Of_Books = fields.total_Number_Of_Books;
        this.in_circulation = fields.in_circulation;

      }
    }
  }
  
  export const BookShema = new Map([
    [Book, {
      kind: "struct",
      fields: [
        ["book_id", "u64"],
        ["total_Number_Of_Books", "u64"],
        ["in_circulation", "u64" ],
      ]
    }]
  ]);


  export class BookCount  { 
    book_counter: number = 0 ;
  
    constructor(fields: { book_counter:number;} | undefined = undefined) {
      if (fields) {
        this.book_counter = fields.book_counter;
      }
    }
  }
  
  export const BookCountShema = new Map([
    [BookCount , {
      kind: "struct",
      fields: [
        ["book_counter", "u64"],
      ]
    }]
  ]);
  
  

  export class Config  { 
    config_accounts1: number = 0 ;
    config_accounts2: number = 0 ;
    config_accounts3: number = 0 ;

    constructor(fields: { config_accounts1:number ; config_accounts2:number;  config_accounts3:number  } | undefined = undefined) {
      if (fields) {
        this.config_accounts1 = fields.config_accounts1;
        this.config_accounts2 = fields.config_accounts2;
        this.config_accounts3 = fields.config_accounts3;

      }
    }
  }
  
  export const ConfigShema = new Map([
    [Config , {
      kind: "struct",
      fields: [
        ["config_accounts1", ["u8",32]],
        ["config_accounts2", ["u8",32]],
        ["config_accounts3", ["u8",32]],

      ]
    }]
  ]);


  
