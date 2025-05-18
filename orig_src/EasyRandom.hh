/*
   copyright 2018 Paul Dreik
   Distributed under GPL v 2.0 or later, at your option.
   See LICENSE for further details.
*/
#ifndef RDFIND_EASYRANDOM_HH_
#define RDFIND_EASYRANDOM_HH_

#include <string>

/**
 * Helper object to "provide a replacement of std::rand()"
 * It is automatically seeded.
 * The state is global, and not held in the class
 * This class is not thread safe.
 */
class EasyRandom final
{
public:
  EasyRandom();
  /**
   * makes N random characters, suitable to use for a random filename.
   * @param N
   * @return
   */
  std::string makeRandomFileString(std::size_t N = 16);

private:
  class GlobalRandom;
  // keep a reference to the global magic static, to avoid the cost of thread
  // safe initialization.
  GlobalRandom& m_rand;

  static GlobalRandom& getGlobalObject();
};

#endif /* RDFIND_EASYRANDOM_HH_ */
