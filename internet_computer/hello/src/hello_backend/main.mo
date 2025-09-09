persistent actor {
  public query func greet(name : Text) : async Text {
    return "This is you first canister. Well done " # name # "!";
  };
};
