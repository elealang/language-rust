//! Definitions
//! A fundamental Rust representation of Elea that is isomorphic to its 
//! formally defined structure. That is, no indexes, runtime additions, or 
//! changes to structure for ease-of-use.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//-----------------------------------------------------------------------------
//  REALITY
//-----------------------------------------------------------------------------

//  TODO 

//--------------------------------------------------------------------------------
//  SPACE
//--------------------------------------------------------------------------------

//  SPACE / Theory
//--------------------------------------------------------------------------------

// Composition in form
#[derive(Serialize, Deserialize)]
pub struct Space {
    pub abstractions: Vec<AbstractionId>,
    pub states:       Vec<StateId>,
    pub arrows:       Vec<Arrow>,
}

// a convenient container for states, but not necessary and not always used
// to reference states uniquely, since they are unique to program space
#[derive(Serialize, Deserialize)]
pub struct AbstractionId(String);

// unique to program space
#[derive(Serialize, Deserialize)]
pub struct StateId(String);

// should be unique in space of (init_state_id, term_state_id)?
#[derive(Serialize, Deserialize)]
pub struct Arrow {
    pub id:          ArrowId,
    pub init_state_id: StateId,
    pub term_state_id: StateId,
}

// if arrows are uniquely determined by (init_state_id, term_state_id) then
// this is simply a useful alias for each pair of those
#[derive(Serialize, Deserialize)]
pub struct ArrowId(String);


// new_table -> joined tables -> rows
// depending on join. arrow that goes fro 
#[derive(Serialize, Deserialize)]
pub struct Set {
    input: Vec<StateId>,  // output / joined table
    output: Vec<StateId>, // input tables / joined tables
}

//  SPACE / Application
//--------------------------------------------------------------------------------

// Addition. Expanasion of space
#[derive(Serialize, Deserialize)]
pub struct Addition {
    pub id: AdditionId,
    pub abstractions: Vec<AbstractionId>,
    pub states: Vec<StateId>,
    pub arrows: Vec<Arrow>,
}

#[derive(Serialize, Deserialize)]
pub struct AdditionId(String);

//--------------------------------------------------------------------------------
//  TIME
//--------------------------------------------------------------------------------

// TIME / Theory
//--------------------------------------------------------------------------------

// time is always partly determined and partly undetermined
#[derive(Serialize, Deserialize)]
pub struct Time {
    pub computers:  Vec<Computer>,
    pub types:      Vec<Type>,
    pub properties: Vec<Property>,
}

#[derive(Serialize, Deserialize)]
pub struct Computer {
    pub id: ComputerId,
    pub type_: TypeId,
    pub properties: Vec<PropertyId>,
}

#[derive(Serialize, Deserialize)]
pub struct ComputerId(String);

#[derive(Serialize, Deserialize)]
pub struct Type {
    pub id: TypeId,
    pub expansion: Vec<Addition>,
    pub extension: Vec<Continuation>,
}

#[derive(Serialize, Deserialize)]
pub struct TypeId(String);

#[derive(Serialize, Deserialize)]
pub struct Continuation {
    pub id: ContinuationId,
    pub function_ref: FunctionId,
}

#[derive(Serialize, Deserialize)]
pub struct ContinuationId(String);

#[derive(Serialize, Deserialize)]
pub struct Property {
    pub id: PropertyId,
}

#[derive(Serialize, Deserialize)]
pub struct PropertyId(String);

// TIME / Application
//--------------------------------------------------------------------------------

// Bridge between space and time
#[derive(Serialize, Deserialize)]
pub struct Computation {
    pub computation_id: ComputationId,
    pub arrow_id: ArrowId,
    pub computer_id: ComputerId,
    pub proofs: Vec<Proof>,
}

#[derive(Serialize, Deserialize)]
pub struct ComputationId(String);

#[derive(Serialize, Deserialize)]
pub struct Proofs {
    pub premises: Vec<Proof>
    pub conclusions: Vec<Proof>
}

#[derive(Serialize, Deserialize)]
pub struct Proof {
    pub id: ProofId,
    pub continuation_id: ContinuationId,
    pub story_id: StoryId,
}

#[derive(Serialize, Deserialize)]
pub struct ProofId(String);

//--------------------------------------------------------------------------------
//  AGENCY
//--------------------------------------------------------------------------------

//  AGENCY / Theory
//--------------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct Agency {
    pub agents: Vec<Agent>,
    pub functions: Vec<Function>,
}

#[derive(Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    pub computers: Vec<ComputerId>,
}

#[derive(Serialize, Deserialize)]
pub struct AgentId(String);

// path / journey / goal / aspiration / 
// data type / constructor
#[derive(Serialize, Deserialize)]
pub struct Function {
    pub function_id: FunctionId,
    pub init_state_id: StateId,
    pub term_state_id: StateId,
}

#[derive(Serialize, Deserialize)]
pub struct FunctionId(String);

#[derive(Serialize, Deserialize)]
pub struct Method {
    pub program_id: ProgramId,
    pub function_id: FunctionId,
}

//  AGENCY / Application
//--------------------------------------------------------------------------------

// story / sequence / trace / log / history / stream / event stream
#[derive(Serialize, Deserialize)]
pub struct Story {
    pub id: StoryId,
    pub agent_id: AgentId
    pub events: Vec<ComputationId>,
}

#[derive(Serialize, Deserialize)]
pub struct StoryId(String);

//--------------------------------------------------------------------------------
//-- PROGRAM
//--------------------------------------------------------------------------------

// intention
#[derive(Deserialize, Serialize)]
pub struct Program {
    pub identity: ProgramIdentity,
    pub theory: Theory,
    pub applications: Vec<Application>,
}

// theories / intention
#[derive(Deserialize, Serialize)]
pub struct Theory {
    pub space: Space,
    pub time: Time,
    pub agency: Agency,
}

// devices / actions
#[derive(Deserialize, Serialize)]
pub struct Application {
    pub additions: Vec<Addition>, // application of sets
    pub computations: Vec<Computation>, // application of stories
    pub stories: Vec<Story>, // application of functions
}

#[derive(Serialize, Deserialize)]
pub struct ProgramIdentity {
    pub id: ProgramId,
    pub name: ProgramName,
    pub description: ProgramDescription,
    pub creator: Agent
}

#[derive(Eq, Deserialize, Hash, PartialEq, Serialize)]
pub struct ProgramId(String);

#[derive(Serialize, Deserialize)]
pub struct ProgramName(String);

#[derive(Serialize, Deserialize)]
pub struct ProgramDescription(String);


//-- | Program
//-- The base program definition. It is almost always within the compositional 
//-- definition
//data Program' = Program' {
    //id      :: ProgramIdentity
  //, space   :: Space
  //, time    :: Time
  //, agency  :: Agency
  //, present :: Reference 
//} deriving (Eq, Generic, Show)

//-- | Program
//-- Instead of composing programs by combining their components, we directly
//-- combine the programs because it's useful to remember that a program is 
//-- explicitly a combinatino of certain other programs
//newtype Program = Program {
  //programById :: HashMap ProgramId [Program']
//} deriving (Eq, Generic, Show)


//-- | Program identity
//data ProgramIdentity = ProgramIdentity {
    //id          :: ProgramId
  //, name        :: ProgramName
  //, description :: ProgramDescription
//} deriving (Eq, Show)


//-- | Vacuous program identity 
//noIdentity :: ProgramIdentity 
//noIdentity = ProgramIdentity {
    //id          = ProgramId ""
  //, name        = ProgramName ""
  //, description = ProgramDescription ""
//}


//-- | Program Id
//newtype ProgramId = ProgramId {
  //value :: Text
//} deriving (Eq, Generic, Show)


//instance ToJSON ProgramId where
  //toJSON (ProgramId s) = JSON.String s

//instance FromJSON ProgramId where
  //parseJSON (JSON.String t) = return $ ProgramId t
  //parseJSON invalid         =
    //prependFailure "parsing ProgramId failed, "
        //(typeMismatch "String" invalid)

//instance Hashable ProgramId


//-- | Program Name
//newtype ProgramName = ProgramName {
  //value :: Text
//} deriving (Eq, Show)


//-- | Program Name
//newtype ProgramDescription = ProgramDescription {
  //value :: Text
//} deriving (Eq, Show)


//-- | Reference
//data Reference = Reference {
    //programId     :: ProgramId
  //, abstractionId :: AbstractionId
  //, stateId       :: StateId
  //, arrowId       :: Maybe ArrowId
//} deriving (Eq, Generic, Show)

//instance ToJSON Reference

//instance FromJSON Reference

//-- | Empty reference
//nowhere :: Reference
//nowhere = Reference {
    //programId     = ProgramId ""
  //, abstractionId = AbstractionId ""
  //, stateId       = StateId ""
  //, arrowId       = Nothing
  //}


//--------------------------------------------------------------------------------
//-- USABILITY
//-- So far we've defined a lot of intersting concepts, but nothing that is 
//-- actually usuable. We need to link the abstraction/conceptual to the reality
//-- that we experience. This leads to explanations... ?
//--------------------------------------------------------------------------------

//-- should this be a monad? somehow seems correct
//--   maybe need to think about what a force does
//--   is kind of like an execute / things happen monad like IO
//--
//-- forces represent causality independently. in Elea of course a function/force
//-- doesn't simply return a value, the value has to go somewhere. it goes into 
//-- the creation of other abstractions which could be computed over by other 
//-- agents. but for a force to be useful we still need to calculate its result
//-- before we put it in its place
//-- is a force = explanation?  is it ironic that I'm defining this to be an 
//-- explanation but I'm having trouble finding an explanation for why it's
//-- the right type?
//-- so a Force is actually defined indp of the other Elea objects
//class Force f d where  
  //execute :: f -> [(d,d)] -> IO d


//-- | Service
//-- how to organize a service by the kind of computer it generates
//-- automatically index properties? 
//-- service implies the computablity of the computers it manages. it is what 
//-- enables the computers to be useful. the "how" of this is never defined in a 
//-- satisfactory way
//-- not just defining how to access the computers, but actually how to call/use
//-- them. this is where the "magic" happens. it is really magic because our 
//-- explanations for how a force works will ultimately leave us unsatisfied
//-- force is an analogy to physical forces e.g. gravity, weak/strong, electromagnetic
//data Service f = Service {
    //-- | The computer computer is computed with a callable with connections
    //-- computation to the "real world" where we need to call a particular 
    //-- computer on some device which needs to be located/accessed. The 
    //-- locating/access/connecting-to of the computer is the computation for the
    //-- computer computer.
    //force     :: f
    //-- | An interface for a computer computer is therefore the list of 
    //-- computers than can be computed by accessing the service. Is also 
    //-- synonymous with an API (application programming interface)
  //, interface :: [Computer]
//} deriving (Eq, Generic, Show)

//type Server = Service
//type Client = Service
//type Index  = Service


//-- | An Application is a usable program where all of the computers required 
//-- by the program are implemented in some service
//-- for example, the Elea UI is an application with develop + elea as programs
//-- and could actually be implemented with any sort of service with relevant
//-- forces. need to compose forces though
//data Application f = Application {
    //program :: Program
  //, service :: Service f
//} deriving (Eq, Generic, Show)

//-- to figure out
//-- how to use service to implement elea program computer
//-- ui
//-- elea ui is really composition of some services to make function
//--   make service is the browser, which computes the application
//--   the application is just a program derived from the main program
//--   the application program is computed by browser computers but other 
//--   parts need other computers available via services
//--   the browser is a service since it provides http/html/css/js computers
//--
//-- need to index serives
//-- find the program service 
//-- genealize both connector/callable and lookup mechanism
//--
//-- backend

