#RustRestrictedIntegers#

A library which attempts to give Rust the ability to have custom types with
limited ranges, ala ada.

## Notes ##
The current approach isn't quite what I want.

Preferrably, you would make a type and then declare items of that type. The max and min
would be qualities of the type, not the items, like they are now. I'm not sure how to do
this. Hopefully, what I learn here will transfer over cleanish to that, while giving
some inspiration onto how to solve that problem.

I think the final solution will be to build a builder struct that builds the LimitVals
I have now?
Inspiration:
https://doc.rust-lang.org/style/ownership/builders.html

Something like:
Builder(min, max, unique name?) -> Type(val, (min, max, unique name set))
Operations done on Type.

(This also only works on uints now, eventually it should use generics.)

## Copying ##

Dual licensed under the Apache License Version 2.0, or the MIT License.
