import Debug "mo:base/Debug";
import Nat "mo:base/Nat";
import Buffer "mo:base/Buffer";

persistent actor HelloWorld {
  transient var greetCounter : Nat = 0;
  transient var visitorsGreeted : Buffer.Buffer<Text> = Buffer.Buffer<Text>(0);

  public func greet(name : Text) : async Text {
    greetCounter += 1;
    visitorsGreeted.add(name);
    Debug.print("Greet method called " # Nat.toText(greetCounter) # " times");
    return "Hello" # name # "! You are visitor: " # Nat.toText(greetCounter);
  };

  public func getGreetCounter() : async Nat {
    return greetCounter;
  };

  public func resetGreetings() : async () {
    greetCounter := 0;
    visitorsGreeted := Buffer.Buffer<Text>(0);
  };

  public func getVisitorsGreeted() : async [Text] {
    return Buffer.toArray(visitorsGreeted);
  };
};
