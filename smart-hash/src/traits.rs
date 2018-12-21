use std::collections::HashSet;

pub trait SmartHash { 
    type option;

    fn into_option(&self) -> Self::option;
}

pub trait SmartHashOpt where Self : Default { }

pub trait SmartHashSet where <Self as SmartHashSet>::output: SmartHash,
{
    type output;

    fn get_matching<'a>(&'a self, options : <<Self as SmartHashSet>::output as SmartHash>::option) -> Option<Vec<&'a Self::output>>;
}

impl<SH> SmartHashSet for HashSet<SH>
    where SH : SmartHash + std::hash::Hash + std::cmp::Eq,
            <SH as SmartHash>::option : std::cmp::PartialEq
{ 
    type output = SH;

    fn get_matching<'a>(&'a self, options : <SH as SmartHash>::option) -> Option<Vec<&'a SH>> {
        let mut matches : Vec<&SH> = Vec::new();

        for item in self.iter() {
            let option = item.into_option();
            if option == options {
                matches.push(&item);
            }
        }

        if matches.len() > 0 {
            Some(matches)
        } else {
            None
        }
    }
}