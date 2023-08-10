// ShoeList.js
import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './ShoeList.css';
import './ShoeOverlay.css';
//import ShoeOverlay from './ShoeOverlay';

function ShoeList() {
  const [shoes, setShoes] = useState([]);
  const [selectedShoe, setSelectedShoe] = useState(null); 
  function handleShoeClick(shoe) {
    console.log('Clicked shoe:', shoe);
    setSelectedShoe(shoe);
  }
  useEffect(() => {
    axios.get('http://127.0.0.1:8000/shoes/api')
      .then(response => {
        setShoes(response.data);
      })
      .catch(error => {
        console.error('Error fetching data:', error);
      });
  }, []);
  const [imageLoaded, setImageLoaded] = React.useState(true);
  function ShoeOverlay({ shoe, onClose }) {
    const gifImage = new Image();
      gifImage.src = 'http://127.0.0.1:8000/shoes/api/gif/'+shoe.style_id;
      gifImage.onload = () => {
    
        setImageLoaded(true);
      };
      gifImage.onerror = () => {
      
        setImageLoaded(false);
      };
    return (
      <div className="overlay">
        <div className="overlay-content">
          <button className="close-button" onClick={onClose}>
            Close
          </button>
          <h2>{shoe.name}</h2>
          {imageLoaded ? (
          <img src={'http://127.0.0.1:8000/shoes/api/gif/'+shoe.style_id} alt={shoe.name} />
        ) : (
          <img src={shoe.image} alt={shoe.name} />
        )}
          {/* Additional information about the shoe */}
        </div>
      </div>
    );
  }
  
   return (
    <div className="shoe-list-container">
      <div className="shoe-list">
        {shoes.map(shoe => (
          <div className="shoe-container" key={shoe.id}>
            <img
              src={shoe.image}
              alt={shoe.name}
              onClick={() => handleShoeClick(shoe)} // Add this line
            />
          </div>
        ))}
      </div>
      {selectedShoe && (
        <ShoeOverlay shoe={selectedShoe} onClose={() => setSelectedShoe(null)} />
      )}
    </div>
  );
}

export default ShoeList;