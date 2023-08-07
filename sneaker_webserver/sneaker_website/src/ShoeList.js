// ShoeList.js
import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './ShoeList.css';
import ShoeOverlay from './ShoeOverlay';

function ShoeList() {
  const [shoes, setShoes] = useState([]);

  useEffect(() => {
    axios.get('http://127.0.0.1:8000/shoes/api')
      .then(response => {
        setShoes(response.data);
      })
      .catch(error => {
        console.error('Error fetching data:', error);
      });
  }, []);

  return (
    <div className="shoe-list-container">
      <div className="shoe-list">
        {shoes.map(shoe => (
          <div className="shoe-container" key={shoe.id}>
           
              <img
                src={shoe.image}
                alt={shoe.name}
              
              />
              <ShoeOverlay shoe={shoe} />
            </div>
          
        ))}
      </div>
    </div>
  );
}

export default ShoeList;