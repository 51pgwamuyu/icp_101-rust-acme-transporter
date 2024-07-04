#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Transporter{
    owner:String,
    id:u64,
    name:String,
    serviceemail:String,
    phonenumber:String,
    route:String,
    trucknumber:String,
    capacityweight:String,
    created_at:u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Complain{
    id:u64,
    complaineremail:String,
    complain:String,
    complainerusername:String,
    created_at:u64,
    

}
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RaiseQuestion{
    id:u64,
    question:String,
    usernamemail:String,
    created_at:u64
}
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct UsersComplainAboutTransporter{
    transportername:String,
    complaineremail:String,
    id:u64,
    complain:String,
    created_at:u64
}
impl Storable for Transporter {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Transporter {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}
impl Storable for Complain {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Complain {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for RaiseQuestion {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for RaiseQuestion {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for UsersComplainAboutTransporter {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for UsersComplainAboutTransporter {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}
//thread
thread_local! {
    static MEMEORY_MANAGER:RefCell<MemoryManager<DefaultMemoryImpl>>=RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static ID_COUNTER:RefCell<IdCell>=RefCell::new(
        IdCell::init(MEMEORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),0).expect("Cannot create a counter")
    );
    static TRANSPORTER_STORAGE:RefCell<StableBTreeMap<u64,Transporter,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMEORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
    static COMPLAIN_STORAGE:RefCell<StableBTreeMap<u64,Complain,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMEORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
    static QUESTION_STORAGE:RefCell<StableBTreeMap<u64,RaiseQuestion,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMEORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
    static USER_COMPLAIN_STORAGE:RefCell<StableBTreeMap<u64,UsersComplainAboutTransporter,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMEORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}


#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]

struct TransporterPayload{
    ownername:String,
    transport_name:String,
    serviceemail:String,
    phonenumber:String,
    route:String,
    trucknumber:String,
    capacityweight:String,
}

#[derive(candid::CandidType,Serialize,Deserialize,Default)]

struct SearchPayload{
    transportername:String,
}

#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct ComplainPayload{
    complaineremail:String,
    complain:String,
    complainerusername:String,

}
#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct RaiseQuestionPayload{
    question:String,
    usernameemail:String,
}
#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]

struct UpdateTransporterPayload{
    ownername:String,
    transport_name:String,
    serviceemail:String,
    phonenumber:String,
    route:String,
    trucknumber:String,
    capacityweight:String,
}
#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct DeletePayload{
    ownerusername:String,
}
#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct UserComplainPayload{
    transportername:String,
    complaineremail:String,
    id:u64,
    complain:String
}
#[derive(candid::CandidType,Deserialize,Serialize)]
enum Errors{
    BusWithSameRegistrationExist{msg:String},
    NotFound{msg:String},
    TansporterNameAlradyEXist{msg:String},
    OnyOwner{msg:String},
    MissingCredentials{msg:String}
}
#[ic_cdk::update]
fn registertransporter(payload: TransporterPayload) -> Result<Transporter, String> {
    // Validate the payload to ensure that the required fields are present
    if payload.serviceemail.is_empty()
        && payload.ownername.is_empty()
        && payload.transport_name.is_empty()
        && payload.phonenumber.is_empty()
        && payload.route.is_empty()
        && payload.trucknumber.is_empty()
        && payload.capacityweight.is_empty()
    {
        return Err("All fields are required".to_string());
    }

    // Validate the payload to ensure that the email format is correct
    if !payload.serviceemail.contains('@') {
        return Err("enter correct email format".to_string());
    }

    // Ensure email address uniqueness and ownername and also transport name
    let email_exists:bool = TRANSPORTER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, val)| val.serviceemail == payload.serviceemail)
    });
    if email_exists {
        return Err("Email already exists".to_string());
    }

   let ownername_exists:bool=TRANSPORTER_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.owner == payload.ownername)
});
if ownername_exists {
    return Err("The usernam already exists".to_string());
}
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let transporter = Transporter {
        id,
        owner:payload.ownername,
        name: payload.transport_name,
        serviceemail:payload.serviceemail,
        phonenumber: payload.phonenumber,
        route:payload.route,
        trucknumber:payload.trucknumber,
        capacityweight:payload.capacityweight,
        created_at: time(),
       
    };

    TRANSPORTER_STORAGE.with(|storage| storage.borrow_mut().insert(id, transporter.clone()));

    Ok(transporter)
}

//Function to retrieve all transporters
#[ic_cdk::query]
fn get_all_transporters() -> Result<Vec<Transporter>, String> {
    TRANSPORTER_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<Transporter> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No transporter found.".to_string())
        } else {
            Ok(records)
        }
    })
}

//function where transporter launch a complain aganist the company
#[ic_cdk::update]
fn transporter_launch_a_complain(payload:ComplainPayload)->Result<Complain, String>{

      // Validate the payload to ensure that the required fields are present
      if payload.complain.is_empty()
      && payload.complaineremail.is_empty()
      && payload.complainerusername.is_empty()
       {
          return Err("All fields are required".to_string());
       }
       // Validate the payload to ensure that the email format is correct
    if !payload.complaineremail.contains('@') {
        return Err("enter correct email format".to_string());
    }

    //check the user owns a truck
    let ownstruck:bool=TRANSPORTER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_,val)| val.owner == payload.complainerusername)
    });
    if !ownstruck {
        return Err("only those that owns trucks can complain".to_string());
    }
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
    let new_coplain=Complain{
        id,
        complainerusername:payload.complainerusername,
        complain:payload.complain,
        complaineremail:payload.complaineremail,
        created_at:time()    };

    COMPLAIN_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_coplain.clone()));

    Ok(new_coplain)
}

//users raise a questuion about a transporter
#[ic_cdk::update]
fn users_raise_question(payload:RaiseQuestionPayload)->Result<RaiseQuestion,String>{


      // Validate the payload to ensure that the required fields are present
      if payload.usernameemail.is_empty()
      || payload.question.is_empty()
       {
          return Err("All fields are required".to_string());
       }
       // Validate the payload to ensure that the email format is correct
    if !payload.usernameemail.contains('@') {
        return Err("enter correct email format".to_string());
    }
    let id = ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("Cannot increment ID counter");
    let new_question=RaiseQuestion{
    id,
    usernamemail:payload.usernameemail,
    question:payload.question,
    created_at:time()
     };
QUESTION_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_question.clone()));

return Ok(new_question);
}

//transporter update details of his truck
#[ic_cdk::update]
fn transporter_update_details(id:u64,payload:UpdateTransporterPayload)->Result<Transporter,String>{
    if payload.serviceemail.is_empty()
        && payload.ownername.is_empty()
        && payload.transport_name.is_empty()
        && payload.phonenumber.is_empty()
        && payload.route.is_empty()
        && payload.trucknumber.is_empty()
        && payload.capacityweight.is_empty()
    {
        return Err("Ensure all credentials are inserted".to_string());
    }
     // Validate the payload to ensure that the email format is correct
     if !payload.serviceemail.contains('@') {
        return Err("Invalid email format".to_string());
    }

match TRANSPORTER_STORAGE.with(|service|service.borrow().get(&id)){
    Some(mut trans)=>{
                        trans.owner=payload.ownername;
                        trans.name=payload.transport_name;
                        trans.serviceemail=payload.serviceemail;
                        trans.phonenumber=payload.phonenumber;
                        trans.capacityweight=payload.capacityweight;
                        trans.route=payload.route;
                        trans.trucknumber=payload.trucknumber;
                        do_insert(&trans);
                        Ok(trans)
                        
    }
    None=>Err("could not update transporter details".to_string()),
}

}

//users search for a  transporter
#[ic_cdk::query]
fn get_a_transporter(payload:SearchPayload)->Result<Transporter,String>{
    
    TRANSPORTER_STORAGE.with(|storage|{
        let transporter=storage.borrow().iter().find(|(_,user)|user.name==payload.transportername);
        match transporter{
            Some((_,record))=>Ok(record.clone()),
            None=>Err("Not Found".to_string()),
        }
    })
}
//transporter remove his truck from company
#[ic_cdk::update]
  fn remove_your_truck_from_company(id:u64,payload:DeletePayload)->Result<Transporter,Errors>{
  //verify payloads
  if payload.ownerusername.is_empty()

   {
      return Err(Errors::MissingCredentials { msg:format! (
        "some credentials are missing",
      ) });
   }
 //verify  transporter is the owner
   //check the user owns a truck
   let transporter_is_owner:bool=TRANSPORTER_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.owner == payload.ownerusername)
});
if !transporter_is_owner{
    return Err(Errors::OnyOwner { msg:format! (
        "you are not authorizd to perform thsi action only owner") });
}
    match TRANSPORTER_STORAGE.with(|storage|storage.borrow_mut().remove(&id)){
        Some(val)=>Ok(val),
        None=>Err(Errors::NotFound { msg:format! (
            "coulde not delete",
        ) })
    }
  }
    //users cpmplain about a transporter
  #[ic_cdk::update]
  fn users_complain(payload:UserComplainPayload)->Result<UsersComplainAboutTransporter,String>{
    if payload.complain.is_empty()
    && payload.complaineremail.is_empty()
    && payload.transportername.is_empty()
     {
        return Err("some fields are missing".to_string());
     }

     //validate transporter
      let itstransporter:bool=TRANSPORTER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_,val)| val.owner == payload.transportername)
    });
    if !itstransporter {
        return Err("the transporter is not registered under acme tranporters".to_string());
    }
     let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
     let new_user_complain=UsersComplainAboutTransporter{
        id,
        complaineremail:payload.complaineremail,
        transportername:payload.transportername,
        complain:payload.complain,
        created_at:time(),
     };
     USER_COMPLAIN_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_user_complain.clone()));
     return Ok(new_user_complain);
  }
//helper unction for updates
fn do_insert(trans:&Transporter){
    TRANSPORTER_STORAGE.with(|service|service.borrow_mut().insert(trans.id,trans.clone()));
}
ic_cdk::export_candid!();