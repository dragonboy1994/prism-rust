const drawVotes = () => {
  voteData = voteData.filter(v => {
    const p = proposerBlocks.find(b => b.blockId===v.to)
    return !p.finalized
  })

  let vote = voteGroup.selectAll('.voteLink').data(voteData, d=>d.id)

  vote.exit().remove()
  vote.enter().append('path')
      .attr('id', d => d.id)
      .attr('class', d => 'voteLink to' + d.to)
      .attr('d', d=>{
        return d.curve
      })
      .style('stroke-width', 3.0)
      .attr('stroke', 'url(#linear-gradient)')
      .style('filter', 'url(#blur)')
      .attr('stroke-dasharray', d => d.totalLength + ' ' + d.totalLength)
        .attr('stroke-dashoffset', d => d.totalLength)
        .transition()
        .duration(t)
      .attr('stroke-dashoffset', 0)
        .on('interrupt', (d) => {
          d3.select('#'+d.id)
            .attr('stroke-dasharray', null)
            .attr('stroke-dashoffset', 0)
            .style('stroke-width', 1.0)
            .style('stroke', '#e6e6e6')
            .style('filter', 'url(#glow)')
         })
        .on('end', (d) => {
          d3.select('#'+d.id)
           .attr('stroke-dasharray', null)
           .attr('stroke-dashoffset', 0)
           .style('stroke-width', 1.0)
           .style('stroke', '#e6e6e6')
           .style('filter', 'url(#glow)')
        })
}


const castVotes = (votingChain, votes) => {
  // Get the last block on voting chain
  const lastBlock = chainsData[votingChain].blocks[chainsData[votingChain].blocks.length-1]
  // Calculate the vote's source coordinate
  if(mock){
    const sourceX = lastBlock.x + width*0.6
    const sourceY = lastBlock.y + proposerBlockSize/2
    let voteToCast = chainsData[votingChain].lastVotedBlock+1
    while(voteToCast<proposerBlockId){
      // Get the proposerBlock to vote for
      const votedProposerBlock = proposerBlocks.find(block => block.blockId==voteToCast)

      // If there are 2 parallel chains, choose 1 block to cast vote for
      if(voteToCast>1){
        const prevProposerBlock = proposerBlocks.find(block => block.blockId==voteToCast-1)
        if(prevProposerBlock===undefined || prevProposerBlock.y===votedProposerBlock.y){
          voteToCast++
          continue
        }
      }

      if(votedProposerBlock===undefined || votedProposerBlock.finalized || !votedProposerBlock.x || Number.isNaN(votedProposerBlock.y)) {
        voteToCast++
        continue
      }
      const targetX = votedProposerBlock.x + width/3 + proposerBlockSize*1.25/2
      const targetY = votedProposerBlock.y + proposerBlockSize/2

      const data = `M${sourceX},${sourceY} Q${sourceX-50},${sourceY-50} ${targetX},${targetY}`
      const voteObj = {from: lastBlock.blockId, to: votedProposerBlock.blockId, fromChain: votingChain, id: 'vote'+lastBlock.blockId+'-'+votedProposerBlock.blockId, curve: `M${sourceX},${sourceY} Q${sourceX-50},${sourceY-50} ${targetX},${targetY}`}
      let tempPath = voteGroup.append('path')
                              .attr('id', 'tempPath')
                              .attr('d', voteObj.curve)
      voteObj.totalLength = tempPath.node().getTotalLength()
      voteGroup.select('#tempPath').remove()
      votedProposerBlock.finalizationLevel+=0.01
      d3.select('#proposerBlock'+votedProposerBlock.blockId)
        .style('fill-opacity', votedProposerBlock.finalizationLevel)
      if(votedProposerBlock.finalizationLevel>finalizationThreshold) confirmBlock(votedProposerBlock)
      voteData.push(voteObj)
      chainsData[votingChain].lastVotedBlock = voteToCast
      voteToCast++
    }
  }
  else {
    // Get the last block on voting chain
    const lastBlock = chainsData[votingChain].blocks[chainsData[votingChain].blocks.length-1]
    // Calculate the vote's source coordinate
    const sourceX = lastBlock.x + 0.6*width
    const sourceY = lastBlock.y + proposerBlockSize/2
    // Cast votes for all blocks until we reach the last voted block, iterating backwards
    let index = proposerBlocks.length-1
    for(let i=0; i<votes.length; i++){
      // Get the proposerBlock to vote for
      const votedProposerBlock = proposerBlocks.find(block => block.blockId==votes[i])

      if(votedProposerBlock===undefined || votedProposerBlock.finalized || !votedProposerBlock.x || Number.isNaN(votedProposerBlock.y)) continue 
      const targetX = votedProposerBlock.x + width/3 + proposerBlockSize*1.25/2
      const targetY = votedProposerBlock.y + proposerBlockSize/2

      const data = `M${sourceX},${sourceY} Q${sourceX-50},${sourceY-50} ${targetX},${targetY}`
      const voteObj = {from: lastBlock.blockId, to: votedProposerBlock.blockId, fromChain: votingChain, id: 'vote'+lastBlock.blockId+'-'+votedProposerBlock.blockId, curve: `M${sourceX},${sourceY} Q${sourceX-50},${sourceY-50} ${targetX},${targetY}`}
      let tempPath = voteGroup.append('path')
                              .attr('id', 'tempPath')
                              .attr('d', voteObj.curve)
      voteObj.totalLength = tempPath.node().getTotalLength()
      voteGroup.select('#tempPath').remove()
      votedProposerBlock.finalizationLevel+=0.01
      d3.select('#proposerBlock'+votes[i])
        .style('fill-opacity', votedProposerBlock.finalizationLevel)
      //if(votedProposerBlock.finalizationLevel>finalizationThreshold) confirmBlock(votedProposerBlock)
      voteData.push(voteObj)
    }
  }
  drawVotes()
}

const mineVotingBlock = (votingChain, blockId, sourceNodeId, parentId, votes) => {
  const check = chainsData[votingChain].blocks.find(b => b.blockId===blockId) 
  if(check==undefined){
    pingNode(sourceNodeId)
    if(votingChain>=numChainsToDisplay) return
    addVotingBlock(votingChain, blockId, sourceNodeId, parentId, votes)
  }
}