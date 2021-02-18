use std::error::Error;

pub mod errors;

#[derive(Debug, Clone)]
pub struct VecMap<K: Clone + PartialEq,V: Clone>(Vec<VecMapElement<K,V>>);

#[derive(Debug, Clone)]
pub struct VecMapElement<K: PartialEq,V: Clone> {
    key: K,
    value: V
}

impl<K: Clone + PartialEq, V: Clone> IntoIterator for VecMap<K, V> {
    type IntoIter = std::vec::IntoIter<VecMapElement<K,V>>;
    type Item = VecMapElement<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K: Clone + PartialEq,V: Clone> VecMap<K,V> {
    pub fn new() -> VecMap<K,V> {
        VecMap(Vec::new())
    }

    pub fn insert(&mut self, key: K, value: V) {
        let key_tmp = key.clone();
        match self.0.iter_mut().find(move |a| a.key == key_tmp) {
            Some(i) => {
                i.value = value.clone();
            }
            None => {
                self.0.push(VecMapElement {
                    key,
                    value
                });
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.0.iter().find(|a| &a.key == key) {
            Some(v) => Some(&v.value),
            None => None
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.0.iter_mut().find(move |a| &a.key == key) {
            Some(v) => Some(&mut v.value),
            None => None
        }
    }

    pub fn remove(&mut self, key: &K) -> Result<(), errors::ElementNotFount> {
        match self.0.iter().position(move |a| &a.key == key) {
            Some(i) => {
                self.0.remove(i);
            }
            None => {
                return Err(errors::ElementNotFount);
            }
        }
        Ok(())
    }

    pub fn change_key(&mut self, key: &K, new_key: &K) -> Result<(), &dyn Error> {
        let key_tmp = key.clone();
        if let Some(_) = self.0.iter().position(move |a| a.key == key_tmp) {
            return Err(&errors::KeyAlreadyExist);
        }
        
        
        match self.0.iter_mut().find(move |a| &a.key == key) {
            Some(i) => {
                i.key = new_key.to_owned();
            }
            None => {
                return Err(&errors::ElementNotFount);
            }
        }
        Ok(())
    }

    pub fn key_exist(&self, key: &K) -> bool {
        match self.0.iter().position(move |a| &a.key == key) {
            Some(_) => true,
            None => false
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {

    use crate::VecMap;

    #[test]
    fn it_works() {
        let mut a: VecMap<i32, i32> = super::VecMap::new();

        a.insert(1, 5);
        a.insert(2, 10);
        a.remove(&1);

        a.into_iter().enumerate().for_each(|(i,e)| {
            println!("{}\t-> {:?}", i,e);
        });
    }
}
