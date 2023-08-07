import React from 'react';
import './ShoeOverlay.css'; // Import your CSS for styling

function ShoeOverlay({ shoe }) {
  return (
    <div className="shoe-overlay">
      <div className="shoe-info">
      <a href={"https://stockx.com" + shoe.link}>
        <h3>{shoe.name}</h3>
        <p>{shoe.style_id}</p>
        <p>{shoe.size}</p>
        <p>{shoe.retail_price}</p>
        
        </a>
      </div>
    </div>
  );
}

export default ShoeOverlay;