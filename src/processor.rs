
use core::borrow;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{ 
    account_info::{next_account_info, AccountInfo}, clock, config, entrypoint::ProgramResult, lamports, msg, program::{invoke, invoke_signed}, program_error::ProgramError, pubkey::{self, Pubkey}, rent::Rent, system_instruction::{self}, system_program, sysvar::Sysvar
    };
    use crate::{instruction::RNGProgramInstruction, state::{Book, BookCount, Config, UserAccount, UserCount}};
    use crate::error::RNGProgramError::{AuthorityError, CollectionoperationError, SubtractionError, OwnershipError, WrongPdaAddressError, NoBooksToDeleteError, NoBooksInCirculation, UnauthorizedUser, BookNotAvailable, InvalidOwnerErr};
    pub struct Processor;
    impl Processor {
    pub fn process(
      _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
      ) -> ProgramResult {
        let instruction: RNGProgramInstruction = RNGProgramInstruction::unpack(instruction_data)?;
    
    
        match instruction { 
          RNGProgramInstruction:: user_count_acc => {
            Self::user_count( accounts,_program_id)
           },
          RNGProgramInstruction:: create_user_acc => {
          Self::create_user_account( accounts,_program_id)
           },
          RNGProgramInstruction:: book_count_acc => {
          Self::book_count( accounts,_program_id)
           },
          RNGProgramInstruction:: create_book_acc => {
          Self::create_book_account( accounts,_program_id)
           },
          RNGProgramInstruction:: delete_book_acc  => {
            Self::delete_book_account( accounts,_program_id)
            },
          RNGProgramInstruction:: borrow_book_acc => {
          Self::borrow_book( accounts,_program_id)
           },
          RNGProgramInstruction:: return_book_acc => {
          Self::return_book( accounts,_program_id)
           },
          RNGProgramInstruction:: add_book_acc{data}  => {
          Self::add_book( accounts,_program_id,data)
           },
          RNGProgramInstruction:: remove_book_acc{data}  => {
          Self::remove_book( accounts,_program_id,data)
           },
          RNGProgramInstruction:: config_acc => {
           Self::create_config( accounts,_program_id)
           },
        
        }
      }

    // Kullanici sayisi
      pub fn user_count (
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let user_counter_account = next_account_info(account_info_iter)?;
  
        let rent = Rent:: default();
        let user_counter_account_rent = rent.minimum_balance(8);
  
        let(counter_address, bump) = Pubkey::find_program_address(&[b"c"], program_id);
  
        invoke_signed ( 
          &system_instruction::create_account(payer.key, &counter_address,user_counter_account_rent , 8, program_id),
          &[user_counter_account.clone(),payer.clone()],
          &[
            &[b"c", &[bump]]
          ]
        )?;
        Ok(())
      }
      
    // Kullanici ekle 
      pub fn create_user_account(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let user_count = next_account_info(account_info_iter)?;
        let config = next_account_info(account_info_iter)?;
        let user_account = next_account_info(account_info_iter)?;

        let config_read = Config:: try_from_slice(&config.data.borrow())?;
      
        if !payer.is_signer{ 
          msg!("payer is not a signer");
          return Err(AuthorityError.into());
        }
  
        if config.owner != program_id{  
          msg!("not a program for authority");
          return Err(OwnershipError.into());

        }
  
        Self::check_authority(payer.key, program_id, config_read)?;

        let mut counter_account_read = UserCount::try_from_slice(&user_count.data.borrow())?; 
        counter_account_read.user_counter =  counter_account_read.user_counter.checked_add(1).ok_or(CollectionoperationError)?; 

        let (user_pda_address, bump) = Pubkey::find_program_address(&[b"user",counter_account_read.user_counter.to_string().as_ref()], program_id);

        let rent = Rent:: default();
        let user_account_rent = rent.minimum_balance(57);

        invoke_signed ( 
        &system_instruction::create_account(payer.key, &user_pda_address, user_account_rent, 57, program_id),
        &[user_account.clone(),payer.clone()],
        &[
          &[b"user", counter_account_read.user_counter.to_string().as_ref(), &[bump]]
        ]

        )?;

        let user_information = UserAccount{
        user_no:counter_account_read.user_counter,
        user_address: payer.key.to_bytes(), 
        book_id: 0,
        borrow_date:0,
        return_date: 0,
          };

      //kullanici bilgileri hesaba yazariz
     user_information.serialize(&mut &mut user_account.try_borrow_mut_data()?[..])?;

     // guncellenmis kullanici saysini hesaba yazariz
     counter_account_read.serialize(&mut &mut user_count.try_borrow_mut_data()?[..])?;

        Ok(()) 
    }

    // Kitap sayisi
    pub fn book_count (
      accounts: &[AccountInfo],
      program_id: &Pubkey,
    ) -> ProgramResult {
      let account_info_iter = &mut accounts.iter();
      let payer = next_account_info(account_info_iter)?;
      let book_counter_account = next_account_info(account_info_iter)?;
       
      let rent = Rent:: default();
      let book_counter_account_rent = rent.minimum_balance(8);

      let(book_counter_address, bump) = Pubkey::find_program_address(&[b"bookcounter"], program_id);

      invoke_signed(&system_instruction::create_account(payer.key, &book_counter_address, book_counter_account_rent, 8, program_id),
      &[book_counter_account.clone(), payer.clone()],
      &[
        &[b"bookcounter", &[bump]]
      ]
        )?;

      Ok(())
    }
    
    //Kitap ekle 
   pub fn create_book_account (
     accounts: &[AccountInfo],
     program_id: &Pubkey,
    ) -> ProgramResult {
      let account_info_iter = &mut accounts.iter();
      let payer = next_account_info(account_info_iter)?;
      let config = next_account_info(account_info_iter)?;
      let book_counter_account = next_account_info(account_info_iter)?;
      let book_account = next_account_info(account_info_iter)?; // Kitap PDA hesabı

      let config_read = Config:: try_from_slice(&config.data.borrow())?;
      
      if !payer.is_signer{ 
        msg!("payer is not a signer");
        return Err(AuthorityError.into());
      }

      if config.owner != program_id{  
        msg!("not a program for authority");
        return Err(OwnershipError.into());

      }

      Self::check_authority(payer.key, program_id, config_read)?;

      let mut book_counter_account_read = BookCount::try_from_slice(&book_counter_account.data.borrow())?;
      book_counter_account_read.book_counter = book_counter_account_read.book_counter.checked_add(1).ok_or(CollectionoperationError)?; // 1 ekledik

      let (book_pda_address,bump) = Pubkey::find_program_address(&[b"Createbook", book_counter_account_read.book_counter.to_string().as_ref()], program_id);

      let rent = Rent:: default();
      let create_book_pda_rent = rent.minimum_balance(24);

      invoke_signed ( 
        &system_instruction::create_account(payer.key, &book_pda_address, create_book_pda_rent, 24, program_id),
        &[book_account.clone(),payer.clone()],
        &[
          &[b"Createbook", book_counter_account_read.book_counter.to_string().as_ref() ,&[bump]]]
       )?;
  
       let book_information = Book{
        book_id:book_counter_account_read.book_counter,
        total_Number_Of_Books:0,
        in_circulation: 0,
    };

    book_information.serialize(&mut &mut book_account.try_borrow_mut_data()?[..])?;
    book_counter_account_read.serialize(&mut &mut book_counter_account.try_borrow_mut_data()?[..])?;

      Ok(())

    }

    //Kitap sil 
   pub fn delete_book_account (
    accounts: &[AccountInfo],
    program_id: &Pubkey,
   ) -> ProgramResult {
     let account_info_iter = &mut accounts.iter();
     let payer = next_account_info(account_info_iter)?;
     let config = next_account_info(account_info_iter)?;
     let book_counter_account = next_account_info(account_info_iter)?;
     let book_account = next_account_info(account_info_iter)?; // Kitap PDA hesabı

      let config_read = Config:: try_from_slice(&config.data.borrow())?;
      
      if !payer.is_signer{ 
        msg!("payer is not a signer");
        return Err(AuthorityError.into());
      }

      if config.owner != program_id{ 
        msg!("not a program for authority");
        return Err(OwnershipError.into());
      }

      Self::check_authority(payer.key, program_id, config_read)?;

     let mut book_counter_account_read = BookCount::try_from_slice(&book_counter_account.data.borrow())?;
     
      if book_counter_account_read.book_counter == 0 {
        msg!("No books to delete");
      return Err(NoBooksToDeleteError.into());
     }

     book_counter_account_read.book_counter = book_counter_account_read.book_counter.checked_sub(1).ok_or(SubtractionError)?; // 1 cikardik


     let (delete_book_pda_address,bump) = Pubkey::find_program_address(&[b"bookDelete", book_counter_account_read.book_counter.to_string().as_ref()], program_id);

     if &delete_book_pda_address != book_account.key {
      msg!("Wrong PDA address");
      return Err(WrongPdaAddressError.into());
     }

     let rent = Rent:: default();
     let create_book_pda_rent = rent.minimum_balance(24);

     invoke_signed ( 
       &system_instruction::create_account(payer.key, &delete_book_pda_address, create_book_pda_rent, 24, program_id),
       &[book_account.clone(),payer.clone()],
       &[
         &[b"bookDelete", book_counter_account_read.book_counter.to_string().as_ref() ,&[bump]]]
      )?;
 
      let book_information = Book{
       book_id:book_counter_account_read.book_counter,
       total_Number_Of_Books:0,
       in_circulation: 0,
   };

   book_information.serialize(&mut &mut book_account.try_borrow_mut_data()?[..])?;
   book_counter_account_read.serialize(&mut &mut book_account.try_borrow_mut_data()?[..])?;

     Ok(())

   }
    
    //Kitap borc alma 
    pub fn borrow_book(
     accounts: &[AccountInfo],
     program_id: &Pubkey,
    ) -> ProgramResult {
      let account_info_iter = &mut accounts.iter();
      let payer = next_account_info(account_info_iter)?;
      let user_account = next_account_info(account_info_iter)?; 
      let book_account  = next_account_info(account_info_iter)?; 

      let mut user_account_data = UserAccount::try_from_slice(&user_account.data.borrow())?;
      let mut borrow_book_data = Book::try_from_slice(&book_account.data.borrow())?;// Kitap PDA'sındaki verileri deserialize edelim

      let clock = clock::Clock::get()?; // solanada saat verisine ulasmak icin kullanilir
      let current_time = clock.unix_timestamp as u64; // UNIX zaman damgası, 1970'ten itibaren geçen saniye sayısını temsil eder.

      if !payer.is_signer {
        msg!("Not the right user");
        return Err(AuthorityError.into());
      }
       if book_account.owner != program_id {
        return Err(InvalidOwnerErr.into());
       }

      if user_account.owner != program_id {
        return Err(InvalidOwnerErr.into());
       }
       
      if borrow_book_data.total_Number_Of_Books == borrow_book_data.in_circulation {
        msg!("Book not available");
        return Err(BookNotAvailable.into());
      }

    // kullanici hesabindaki user address ile borc alan kullanici ayni kisi mi 
      if user_account_data.user_address != payer.key.to_bytes() {
        msg!("User address does not match");
        return Err(UnauthorizedUser.into());
      }

      // kutuphanede hic kitap yoksa
      if borrow_book_data.total_Number_Of_Books == 0 {
        msg!("There is no book. you can't borrow");
        return Err(NoBooksToDeleteError.into());
      }

      user_account_data.borrow_date = user_account_data.borrow_date.checked_add(current_time).ok_or(CollectionoperationError)?;
      user_account_data.borrow_date = user_account_data.borrow_date.checked_add(259200).ok_or(CollectionoperationError)?;

    // Eger kitap mevcutsa borç alındı sayısını güncelleyelim
       borrow_book_data.in_circulation = borrow_book_data.in_circulation.checked_add(1).ok_or(CollectionoperationError)?; // kitap artik borc alindi
      
      borrow_book_data.serialize(&mut &mut book_account.data.borrow_mut()[..])?;

      Ok(())
    }
   
    //Kitap teslim
    pub fn return_book (
      accounts: &[AccountInfo],
      program_id: &Pubkey,
    ) -> ProgramResult {
      let account_info_iter = &mut accounts.iter();
      let payer = next_account_info(account_info_iter)?;
      let user_account = next_account_info(account_info_iter)?; 
      let book_account  = next_account_info(account_info_iter)?; 

      let (return_address,bump) = Pubkey::find_program_address(&[b"return"], program_id);

      let mut return_book_data = Book::try_from_slice(&book_account.data.borrow())?;

      // dolasimdaki kitap sayisini kontrol edelim
      if return_book_data.in_circulation == 0 {
        msg!("No books in circulation to return.");
        return Err(NoBooksInCirculation.into());
      }
      
      if book_account.owner != program_id {
        return Err(InvalidOwnerErr.into());
       }

      if user_account.owner != program_id {
        return Err(InvalidOwnerErr.into());
       }
      return_book_data.in_circulation = return_book_data.in_circulation.checked_sub(1).ok_or(SubtractionError)?; // kitap teslim edildi

      return_book_data.serialize(&mut &mut book_account.data.borrow_mut()[..])?;

  Ok(())
}

   // Disardan sisteme mevcut kitap turunden kitap ekleme 
    pub fn add_book (
      accounts: &[AccountInfo],
      program_id: &Pubkey,
      data: u64,
    ) -> ProgramResult {
      let account_info_iter = &mut accounts.iter();

      let payer = next_account_info(account_info_iter)?;
      let book_account = next_account_info(account_info_iter)?; 
      let config = next_account_info(account_info_iter)?;

      let config_read = Config:: try_from_slice(&config.data.borrow())?;
         
      if !payer.is_signer{ 
        msg!("payer is not a signer");
        return Err(AuthorityError.into());
      }

      if config.owner != program_id{  
        msg!("not a program for authority");
        return Err(OwnershipError.into());

      }

      Self::check_authority(payer.key, program_id, config_read)?;

     let mut book_account_read = Book::try_from_slice(&book_account.data.borrow())?;

     book_account_read.total_Number_Of_Books = book_account_read.total_Number_Of_Books.checked_add(data).ok_or(CollectionoperationError)?;

      book_account_read.serialize(&mut &mut book_account.data.borrow_mut()[..])?;

      Ok(())
    }

   // Disardan sisteme mevcut kitap turunden kitap silme 
    pub fn remove_book (
      accounts: &[AccountInfo],
      program_id: &Pubkey,
      data: u64,
    ) -> ProgramResult {
      let account_info_iter = &mut accounts.iter();

      let payer = next_account_info(account_info_iter)?;
      let book_account = next_account_info(account_info_iter)?; 
      let config = next_account_info(account_info_iter)?;

      let config_read = Config:: try_from_slice(&config.data.borrow())?;
         
      if !payer.is_signer{ 
        msg!("payer is not a signer");
        return Err(AuthorityError.into());
      }

      if config.owner != program_id{  
        msg!("not a program for authority");
        return Err(OwnershipError.into());

      }

      Self::check_authority(payer.key, program_id, config_read)?;

     let mut remove_book_account = Book::try_from_slice(&book_account.data.borrow())?;

     // silinecek kitap yoksa
     if remove_book_account.total_Number_Of_Books == 0 {
      msg!("There is no book. you can't borrow");
      return Err(NoBooksToDeleteError.into());
    }
     remove_book_account.total_Number_Of_Books =remove_book_account.total_Number_Of_Books.checked_sub(data).ok_or(SubtractionError)?;

     remove_book_account.serialize(&mut &mut book_account.data.borrow_mut()[..])?;

      Ok(())
     
    }

    // Gelen hesap otorite mi?
    pub fn check_authority(
      authority: &Pubkey,
      program_id: &Pubkey,
      config: Config,
    ) -> ProgramResult {
    
      if config.config_accounts1 != authority.to_bytes()
      && config.config_accounts2 != authority.to_bytes()
      && config.config_accounts3 != authority.to_bytes() {
       
        msg!("the user is the not authority");
        return Err((AuthorityError.into()));
      }
      Ok(())
    }

    // Otorite olustur
    pub fn create_config (
      accounts: &[AccountInfo],
      program_id: &Pubkey,
    ) -> ProgramResult{

      let account_info_iter = &mut accounts.iter();
      let payer = next_account_info(account_info_iter)?;
      let config = next_account_info(account_info_iter)?;

      let(config_address, bump) = Pubkey::find_program_address(&[b"config"], program_id);

      let rent = Rent:: default();
      let lamports = rent.minimum_balance(96);

      invoke_signed ( 
        &system_instruction::create_account(payer.key, &config_address, lamports, 96, program_id),
        &[config.clone(),payer.clone()],
        &[
          &[b"config" ,&[bump]]]
       )?;
       
       let config_data = Config{ 
        config_accounts1:payer.key.to_bytes(), 
        config_accounts2: payer.key.to_bytes(),
        config_accounts3:payer.key.to_bytes() 
        };

       config_data.serialize(&mut &mut config.data.borrow_mut()[..])?;
       
      Ok(())
    }

    // Otorite guncelle    
    pub fn update_config (
      accounts: &[AccountInfo],
      program_id: &Pubkey,
    ) -> ProgramResult{

      let account_info_iter = &mut accounts.iter();
      let payer = next_account_info(account_info_iter)?;
      let config = next_account_info(account_info_iter)?;
      let authority1 = next_account_info(account_info_iter)?;
      let authority2 = next_account_info(account_info_iter)?;
       
        if !payer.is_signer { 
        msg!("payer is not a signer");
        return Err(AuthorityError.into());
      }

      if config.owner != program_id{  
        msg!("not a program for authority");
        return Err(OwnershipError.into());

      }
       let config_read = Config::try_from_slice(&config.data.borrow())?;
        
       Self::check_authority(payer.key, program_id, config_read)?; // yetkiyi kontrol eder

       let config_data = Config { 
        config_accounts1:payer.key.to_bytes(), 
        config_accounts2: authority1.key.to_bytes(), 
        config_accounts3: authority2.key.to_bytes() 
      };

       config_data.serialize(&mut &mut config.data.borrow_mut()[..])?;
       
      Ok(())
    }
    
  }
